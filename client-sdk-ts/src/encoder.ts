export class Encoder {
    public readonly buffer: Uint8Array;
    private index = 0;

    constructor(length: number | Uint8Array) {
        this.buffer = typeof length === "number"
            ? new Uint8Array(length)
            : length;
    }

    writeU8(v: number) {
        if(this.index + 1 > this.buffer.length) return;
        this.buffer[this.index++] = v;
    }

    writeU16LE(v: number) {
        if(this.index + 2 > this.buffer.length) return;
        this.buffer[this.index++] = v;
        this.buffer[this.index++] = v >> 8;
    }

    writeU64LE(v: number) {
        if(this.index + 8 > this.buffer.length) return;

        this.buffer[this.index++] = v;
        this.buffer[this.index++] = v >> 8;
        this.buffer[this.index++] = v >> 16;
        this.buffer[this.index++] = v >> 24;
        this.buffer[this.index++] = 0;
        this.buffer[this.index++] = 0;
        this.buffer[this.index++] = 0;
        this.buffer[this.index++] = 0;
        // TODO: u64
    }

    writeI64LE(v: number) {
        if(this.index + 8 > this.buffer.length) return;
        this.buffer[this.index++] = v;
        this.buffer[this.index++] = v >> 8;
        this.buffer[this.index++] = v >> 16;
        this.buffer[this.index++] = 0;
    }

    goTo(index: number) {
        this.index = index;
    }

    skip(n: number) {
        this.index += n;
    }
}