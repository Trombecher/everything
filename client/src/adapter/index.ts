import {Decoder} from "./decoder.ts";
import {Constraint, matchesConstraint} from "./constraints.ts";
import {Encoder} from "./encoder.ts";
import {
    EVENT_OBJECT_DELETE, EventId,
    MIN_TEMP_PENDING_ID,
    ObjectId,
    PermissionLevel,
    PL_ADMIN, RawValue, RPC_OBJECTS_CREATE, RPC_OBJECTS_EXISTS, RPC_OBJECTS_QUERY,
    RPCId,
    SESSION_ID_LENGTH,
} from "./model.ts";
import {BUILT_IN_TAG_ID_CREATED, isBuiltInTagId, MIN_OBJECT_ID} from "./objects.ts";
import {Language} from "./lang.ts";
import {BUILT_IN_NAMES_MAP} from "../translations.ts";

const QUERY_BATCH_SIZE = 16;

export class Client {
    /**
     * Caches all data received from the server.
     *
     * This is **not** a mirror of all data on the server. And
     * this also means that if the cache does not have it, it does not mean
     * it does not exist.
     *
     * @private
     */
    private readonly cache: Map<number, Record<number, RawValue | undefined>> = new Map();
    private nextPendingId: number = MIN_TEMP_PENDING_ID;

    /**
     * The next object id, relevant for local object creation.
     * @private
     */
    private nextObjectId: number = MIN_OBJECT_ID;

    // Handlers
    public onObjectCreated: ((oid: ObjectId) => any) | undefined;
    public onAssociation: ((oid: ObjectId, tag: ObjectId, optionalValue: any) => any) | undefined;
    public onObjectDeleted: ((oid: ObjectId) => any) | undefined;
    public onClose: (() => any) | undefined;

    private constructor(
        private readonly ws: WebSocket | undefined,
        private readonly pl: PermissionLevel,
        private readonly pending: Map<number, (decoder: Decoder) => void>,
    ) {
    }

    isLocal(): boolean {
        return !this.ws;
    }

    /**
     * Creates a new Client
     * @param userName
     * @param passkey
     * @param pl The maximum permission level for this session
     */
    static async create(
        userName: string,
        passkey: Uint8Array,
        pl: PermissionLevel = PL_ADMIN,
    ) {
        const ws = new WebSocket("/connect");

        if(await new Promise(res => {
            ws.onopen = () => res(true);
            ws.onerror = () => res(false);
        })) {
            return; // TODO: better error handling
        }

        const loginPayload = new Uint8Array(8 + userName.length + 8 + passkey.length);

        const encodedSessionIDAndPl: Uint8Array | undefined = await new Promise(res => {
            // Send userName + passkey
            ws.send(loginPayload);

            ws.onmessage = e => {
                if(!(e.data instanceof ArrayBuffer)) res(undefined);
                res(new Uint8Array(e.data));
            };

            ws.onclose = () => res(undefined);
        });

        // TODO: better error handling
        if(!encodedSessionIDAndPl) return;
        const decoder = new Decoder(encodedSessionIDAndPl);

        if(decoder.readU8()) return;

        const sessionID = decoder.readFixedU8Array(SESSION_ID_LENGTH);
        if(!sessionID) return;

        // @ts-ignore
        pl = decoder.readU8();
        if(!pl) return;

        const userId = decoder.readU64LE();
        if(!userId) return;

        if(!decoder.isAtEnd()) return;

        const pending = new Map();

        ws.onmessage = e => {
            if(!(e.data instanceof ArrayBuffer)) return; // TODO: error

            const decoder = new Decoder(new Uint8Array(e.data));

            const msgId = decoder.readI64LE();
            if(!msgId) return; // TODO: error

            if(msgId >= MIN_TEMP_PENDING_ID) {
                // This is an answer from the server.
                const callback = pending.get(msgId);

                if(callback) {
                    callback(decoder);
                    pending.delete(msgId);
                } else {
                    // TODO: Diagnostic, received an answer for no question.
                }
            }

            // TODO: handle events
        };

        return new Client(ws, pl, pending);
    }

    /**
     * Invokes an RPC. Returns with the payload. Must have 16 unused bytes preallocated at the beginning of the encoder.
     */
    private async invokeRPC(id: RPCId, encoder: Encoder): Promise<Decoder> {
        if(!this.ws) throw new Error("called `invokeRPC(...)` on a local client");

        const commId = this.nextPendingId++;

        encoder.goTo(0);
        encoder.writeI64LE(id);
        encoder.writeU64LE(commId);

        return new Promise(res => {
            this.pending.set(commId, res);
            this.ws!.send(encoder.buffer);
        });
    }

    private sendEvent(eventId: EventId, encoder: Encoder) {
        if(!this.ws) throw new Error("called `sendEvent(...)` on a local client");

        encoder.goTo(0);
        encoder.writeI64LE(eventId);
        this.ws.send(encoder.buffer);
    }

    async createObject(): Promise<ObjectId | undefined> {
        // TODO: perms

        const now = new Date().valueOf();

        let oid;

        if(this.ws) {
            const decoder = await this.invokeRPC(RPC_OBJECTS_CREATE, new Encoder(16));
            oid = decoder.readI64LE();
            if(oid === undefined) return; // TODO: Error

            if(!decoder.isAtEnd()) return;
        } else {
            oid = this.nextObjectId++;
        }

        this.cache.set(oid, {[BUILT_IN_TAG_ID_CREATED]: now});
        this.onObjectCreated?.(oid);

        return oid;
    }

    async getTagName(tagId: ObjectId, language: Language): Promise<string | undefined> {
        if(isBuiltInTagId(tagId)) return BUILT_IN_NAMES_MAP[language][tagId];

        throw new Error("TODO");
    }

    deleteObject(objectId: number) {
        this.cache.delete(objectId);

        const encoder = new Encoder(16);
        encoder.skip(8);
        encoder.writeU64LE(objectId);

        this.sendEvent(EVENT_OBJECT_DELETE, encoder);
    }

    async exists(oid: ObjectId) {
        if(this.cache.has(oid)) return true;

        const encoder = new Encoder(24);
        encoder.skip(16);
        encoder.writeI64LE(oid);

        const decoder = await this.invokeRPC(RPC_OBJECTS_EXISTS, encoder);
        const bool = decoder.readU8();
        if(!bool) return;

        if(!decoder.isAtEnd()) return;

        this.cache.set(oid, {});

        return bool == 1;
    }

    async* queryBatched(c: Constraint | undefined, batchSize: number) {
        if(batchSize < 1) throw new Error("`batch_size` must be positive");

        let batch: ObjectId[] = [];

        for(const [id, obj] of this.cache.entries()) {
            if(batch.length === batchSize) {
                yield batch;
                batch = [];
            }

            if(matchesConstraint(obj, c)) batch.push(id);
        }

        // Yield the remaining local objects.
        if(batch.length) yield batch;

        if(!this.ws) return;

        for(let offset = 0; ; offset += batchSize) {
            const encoder = new Encoder(26);
            encoder.skip(16);
            encoder.writeU16LE(batchSize); // Limit
            encoder.writeU64LE(offset); // Offset

            const decoder = await this.invokeRPC(RPC_OBJECTS_QUERY, encoder);

            batch = [];

            for(const id of decoder.iterI64Array()) {
                batch.push(id);
                this.cache.set(id, {});
            }

            yield batch;

            if(batch.length < batchSize) break;
        }
    }

    async* query(c?: Constraint, batchSize = QUERY_BATCH_SIZE): AsyncGenerator<ObjectId> {
        for await(const batch of this.queryBatched(c, batchSize)) {
            for(const id of batch) yield id;
        }
    }

    async* tags(id: ObjectId) {
        if(this.ws) throw new Error("NOT IMPL");

        const o = this.cache.get(id);
        if(!o) return;

        for(const x in o) yield +x;
    }

    async has(objectId: ObjectId, tag: ObjectId): Promise<boolean> {
        // TODO: tag validation

        if(this.ws) throw new Error("NOT IMPL"); // TODO: impl

        const obj = this.cache.get(objectId);
        if(!obj) return false;

        return tag in obj;
    }

    async associate(objectId: ObjectId, tag: ObjectId, value: RawValue | undefined) {
        if(this.ws) throw new Error("TODO NOT IMPL"); // TODO

        // TODO: check tag constraint

        const object = this.cache.get(objectId);
        if(!object) return;

        object[tag] = value;

        this.onAssociation?.(objectId, tag, value);
    }

    /**
     * Gets the associated value of the tagId with the objectId.
     */
    async get(objectId: ObjectId, tagId: ObjectId): Promise<RawValue | undefined> {
        if(this.ws) throw new Error("TODO"); // TODO

        const obj = this.cache.get(objectId);
        if(!obj) return;

        return obj[tagId];
    }

    clearCache() {
        this.cache.clear();
    }

    async terminateEverySession() {
        // TODO: must be admin
        if(this.pl != PL_ADMIN) return;
    }

    /**
     * Creates a local client environment.
     */
    static local() {
        return new Client(
            undefined,
            PL_ADMIN,
            new Map(),
        );
    }
}