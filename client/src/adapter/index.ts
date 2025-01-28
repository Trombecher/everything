import {Decoder} from "./decoder.ts";
import {Constraint, matchesConstraint} from "./constraints.ts";
import {Encoder} from "./encoder.ts";
import {
    EVENT_OBJECT_DELETE, EventId,
    MIN_TEMP_PENDING_ID,
    ObjectId,
    PermissionLevel,
    PL_ADMIN, RPC_OBJECTS_CREATE, RPC_OBJECTS_EXISTS, RPC_OBJECTS_QUERY,
    RPCId,
    SESSION_ID_LENGTH, Value,
} from "./model.ts";

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
    private readonly cache: Map<number, Record<number, any>> = new Map();
    private nextPendingId: number = MIN_TEMP_PENDING_ID;

    // Handlers
    public onObjectCreated: (oid: ObjectId) => any | undefined;
    public onAssociation: (oid: ObjectId, tag: ObjectId, optionalValue: any) => any | undefined;
    public onObjectDeleted: (oid: ObjectId) => any | undefined;
    public onClose: () => any | undefined;

    private constructor(
        private readonly ws: WebSocket,
        private readonly pl: PermissionLevel,
        private readonly pending: Map<number, (decoder: Decoder) => void>,
    ) {
    }

    /**
     * Creates a new Client
     * @param userName
     * @param passkey
     * @param pl The maximum permission level for this session
     */
    async create(
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
                const callback = this.pending.get(msgId);

                if(callback) {
                    callback(decoder);
                    this.pending.delete(msgId);
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
        const commId = this.nextPendingId++;

        encoder.goTo(0);
        encoder.writeI64LE(id);
        encoder.writeU64LE(commId);

        return new Promise(res => {
            this.pending.set(commId, res);
            this.ws.send(encoder.buffer);
        });
    }

    private sendEvent(eventId: EventId, encoder: Encoder) {
        encoder.goTo(0);
        encoder.writeI64LE(eventId);
        this.ws.send(encoder.buffer);
    }

    async createObject(): Promise<ObjectId | undefined> {
        // TODO: perms

        const decoder = await this.invokeRPC(RPC_OBJECTS_CREATE, new Encoder(16));
        const oid = decoder.readI64LE();
        if(oid === undefined) return; // TODO: Error

        if(!decoder.isAtEnd()) return;

        this.cache.set(oid, {});

        return oid;
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

    async* query(c?: Constraint, batch_size = QUERY_BATCH_SIZE): AsyncGenerator<ObjectId> {
        if(batch_size < 1) throw new Error("`batch_size` must be positive");

        for(const [id, obj] of this.cache.entries()) {
            if(matchesConstraint(obj, c)) yield id;
        }

        for(let offset = 0;; offset += batch_size) {
            const encoder = new Encoder(26);
            encoder.skip(16);
            encoder.writeU16LE(batch_size); // Limit
            encoder.writeU64LE(offset); // Offset

            const decoder = await this.invokeRPC(RPC_OBJECTS_QUERY, encoder);

            let n = 0;

            for(const id of decoder.iterI64Array()) {
                yield id;
                this.cache.set(id, {});
                ++n;
            }

            if(n < batch_size) break;
        }
    }

    get(objectId: ObjectId, tagId: ObjectId): Value | undefined {

    }

    clearCache() {
        this.cache.clear();
    }

    async terminateEverySession() {
        // TODO: must be admin
        if(this.pl != PL_ADMIN) return;
    }
}