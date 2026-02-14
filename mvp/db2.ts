type Association = { target: Id, tag: Id, value: Id };

type Structure = Association[];

type Id = { type: "self" }
    | { type: "inline-data", data: bigint }
    | { type: "data-hash", hash: bigint }
    | { type: "structure", structure: Structure }

export const idsEqual = (ida: Id, idb: Id) => {
    if (ida.type === "self" && idb.type === "self") return true;
    if (ida.type === "inline-data" && idb.type === "inline-data") return ida.data === idb.data;
    if (ida.type === "data-hash" && idb.type === "data-hash") return ida.hash === idb.hash;
    if (ida.type === "structure" && idb.type === "structure") return ida.
}

export class Database {
    private readonly storedAssociations: Association[];


}