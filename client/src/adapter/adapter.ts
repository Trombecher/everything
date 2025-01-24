import {readU64LE, readU8LE, writeU64LE} from "./encode";
import {ObjectType__tryFrom} from "./model.ts";

type Adapter = {w: WebSocket, n: number, r: Map<number, (payload: Uint8Array) => void>};

export const Adapter__new = async(userName: string, passkey: Uint8Array): Promise<Adapter> => {
    const ws = new WebSocket("/connect");

    // Wait for ready
    await new Promise(res => ws.onopen = res);

    /*
    const initialRequest = new Uint8Array(8 + userName.length + 8 + passkey.length);
    writeU64LE(initialRequest, 0, userName.length);

    ws.onmessage = () => {

    };

    ws.send(initialRequest);
     */

    const requestCallbackMap = new Map<number, (payload: Uint8Array) => void>();

    ws.binaryType = "arraybuffer";
    ws.onmessage = async e => {
        const buffer = new Uint8Array(e.data);

        const rid = readU64LE(buffer, 0);
        const callback = requestCallbackMap.get(rid);

        if(!callback) {
            console.warn(`Got payload for a non-existing request. Request ID: ${rid}`);
            return;
        }

        requestCallbackMap.delete(rid);
        callback(buffer.slice(8));
    };

    return {w: ws, n: 0, r: requestCallbackMap};
};

const Adapter__call = async(adapter: Adapter, pid: number, payload: Uint8Array) => {
    const n = adapter.n++;
    writeU64LE(payload, 0, n);

    // Write PID
    writeU64LE(payload, 8, pid);

    return await new Promise((res: (value: Uint8Array) => void) => {
        adapter.r.set(n, res);
        adapter.w.send(payload);
    });
};

const OBJECTS_ITER_CHUNK_SIZE = 1;

export const Adapter__objects__iter = async function* (adapter: Adapter) {
    const request = new Uint8Array(8 + 8 + 8 + 8);
    let chunkIndex = 0;

    while(1) {
        writeU64LE(request, 16, OBJECTS_ITER_CHUNK_SIZE); // Limit
        writeU64LE(request, 24, chunkIndex * OBJECTS_ITER_CHUNK_SIZE); // Offset

        const res = await Adapter__call(adapter, 0, request);

        const requestStatus = readU8LE(res, 0);
        if(requestStatus) throw new Error(`Request status: ${requestStatus}`);

        const objectCount = readU64LE(res, 1);
        const chunkData = res.slice(9);

        if(objectCount > OBJECTS_ITER_CHUNK_SIZE) {
            console.warn(`The server sent more objects (${objectCount}) than requested (${OBJECTS_ITER_CHUNK_SIZE}).`)
        }

        for(let objectIndex = 0; objectIndex < objectCount; ++objectIndex) {
            const objectID = readU64LE(chunkData, objectIndex * 9);
            const rawObjectType = readU8LE(chunkData, objectIndex * 9 + 8);
            const objectType = ObjectType__tryFrom(rawObjectType);

            if(objectType) {
                yield {id: objectID, type: objectType};
            } else {
                console.error(`Received invalid object type with integer value ${rawObjectType} (on object id ${objectID}). Ignoring malformed object.`);
            }
        }

        if(objectCount < OBJECTS_ITER_CHUNK_SIZE) {
            // No more objects
            break;
        }

        chunkIndex++;
    }
};

export const Adapter__close = async(adapter: Adapter) => {
    await new Promise(res => {
        adapter.w.onclose = res;
        adapter.w.close();
    });
};