export type Id = string;

export type Object = Structure | Id;

export type Structure = {[key: Id]: Object | [Object, Object, ...Object[]]}

