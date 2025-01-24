export const enum ObjectType {
    File = 1,
    Directory = 2,
}

export const ObjectType__tryFrom = (value: number) => {
    if(value < 1 || value > 2) return undefined;
    return value as ObjectType;
}

export type Object = {id: number, type: ObjectType};