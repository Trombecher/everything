const textDecoder = new TextDecoder();

export class Decoder {
    private index = 0;

    constructor(public readonly data: Uint8Array) {
    }

    readU8(): number | undefined {
        if(this.index + 1 > this.data.length) return;
        return this.data[this.index++];
    }

    readU32LE(): number | undefined {
        if(this.index + 4 > this.data.length) return;
        return this.data[this.index++]
            | (this.data[this.index++] << 8)
            | (this.data[this.index++] << 16)
            | (this.data[this.index++] << 24);
    }

    readU64LE(): number | undefined {
        const a = this.readU32LE();
        if(a === undefined) return a;
        const b = this.readU32LE();
        if(b === undefined) return b;
        return a + (b * 2 ** 32);
    }

    readI64LE(): number | undefined {
        return this.readU64LE(); // TODO: BUG HERE
    }

    readU8Array() {
        const len = this.readU64LE();
        if(len === undefined) return;

        if(this.index + len > this.data.length) return;

        const buf = this.data.slice(this.index, this.index + len);
        this.index += len;
        return buf;
    }

    readString() {
        const buf = this.readU8Array();
        if(buf === undefined) return;

        try {
            return textDecoder.decode(buf);
        } catch(_) {
            return;
        }
    }

    readFixedU8Array(n: number) {
        if(this.index + n > this.data.length) return;

        const buf = this.data.slice(this.index, this.index + n);
        this.index += n;
        return buf;
    }

    *iterI64Array() {
        const len = this.readU64LE();
        if(len === undefined) return;

        if(this.index + len * 8 > this.data.length) return;

        for(let i = 0; i < len; i++, this.index += 8) {
            yield this.readI64LE()!;
        }
    }

    isAtEnd() {
        return this.index === this.data.length;
    }
}