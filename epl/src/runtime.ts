export type ObjectId = bigint;

export type Value = readonly ObjectId[];

export const add = (a: Value, b: Value) =>
    Array.from(
        {length: Math.max(a.length, b.length)},
        (_, i) => (a[i] ?? 0n) + (b[i] ?? 0n),
    );

/*

* int_add
* int_subtract
* int_or
* int_and
* int_xor
* int_multiply
* int_divide
* int_remainder

*/
