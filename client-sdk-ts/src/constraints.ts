import {ObjectId, Value} from "./model.ts";

export const CS_AND = 1;
export const CS_OR = 2;
export const CS_NOT = 3;
export const CS_LEAF = 4;

export type Constraint = {
    _: typeof CS_AND,
    l: Constraint,
    r: Constraint
} | {
    _: typeof CS_OR,
    l: Constraint,
    r: Constraint
} | {
    _: typeof CS_NOT,
    i: Constraint
} | {
    _: typeof CS_LEAF,
    i: ObjectId,
    v?: Value
};

export const matchesConstraint = (obj: Record<number, any>, c?: Constraint): boolean => {
    if(!c) return true;

    switch(c._) {
        case CS_AND: return matchesConstraint(obj, c.l) && matchesConstraint(obj, c.r);
        case CS_OR: return matchesConstraint(obj, c.l) || matchesConstraint(obj, c.r);
        case CS_NOT: return matchesConstraint(obj, c.i);
    }

    return c.i in obj && (!c.v || obj[c.i] === c.v.v);
}