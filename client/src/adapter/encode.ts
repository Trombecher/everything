export const writeU64LE = (array: Uint8Array, index: number, value: number) => {
    // TODO: fix u64
    array[index] = value;
    array[index + 1] = value >> 8;
    array[index + 2] = value >> 16;
    array[index + 3] = value >> 24;
    array[index + 4] = 0;
    array[index + 5] = 0;
    array[index + 6] = 0;
    array[index + 7] = 0;
};

export const readU64LE = (array: Uint8Array, index: number) => {
    // TODO: fix u64
    return array[index]
        | (array[index + 1] << 8)
        | (array[index + 2] << 16)
        | (array[index + 3] << 24);
}

export const readU8LE = (array: Uint8Array, index: number) => {
    return array[index]
}