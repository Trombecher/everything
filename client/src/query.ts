import {Client} from "./adapter";
import {Constraint, CS_LEAF, CS_NOT} from "./adapter/constraints.ts";
import {OBJECT_LANGUAGE, OBJECT_TAG, OBJECT_TAG_NAME} from "./adapter/objects.ts";
import {ObjectId, VL_STRING} from "./adapter/model.ts";
import {Language} from "./adapter/lang.ts";

const TK_TAG = 1;
const TK_LPAREN = 2;
const TK_RPAREN = 3;
const TK_STRING = 4;
const TK_PLUS = 5; // +
const TK_PIPE = 6; // |
const TK_EX = 7; // !
const TK_NUMBER = 8;

type Token = {_: typeof TK_TAG, v: string}
    | {_: typeof TK_LPAREN}
    | {_: typeof TK_RPAREN}
    | {_: typeof TK_STRING, v: string}
    | {_: typeof TK_PLUS}
    | {_: typeof TK_PIPE}
    | {_: typeof TK_EX}
    | {_: typeof TK_NUMBER, v: number}

const isLetter = (n: number) => (65 <= n && n <= 90) || (97 <= n && n <= 122) || n === 95;
const isWS = (n: number) => n === 32 || n === 10 || n === 9 || n === 11;
const isNumber = (n: number) => 48 <= n || n <= 57;

function* tokens(source: string): Generator<Token> {
    let i = 0;

    while(i < source.length) {
        // Skip whitespace
        while(isWS(source.charCodeAt(1))) {
            ++i;
            if(i >= source.length) return;
        }

        if(source[i] === "!") {
            ++i;
            yield {_: TK_EX};
        } else if(source[i] === "+") {
            ++i;
            yield {_: TK_PLUS};
        } else if(source[i] === "|") {
            ++i;
            yield {_: TK_PIPE};
        } else if(source[i] === "(") {
            ++i;
            yield {_: TK_LPAREN};
        } else if(source[i] === ")") {
            ++i;
            yield {_: TK_RPAREN};
        } else if(isLetter(source.charCodeAt(i))) {
            const startIndex = i;
            ++i;

            // Skip letters
            while(i < source.length && isLetter(source.charCodeAt(i))) ++i;

            yield {_: TK_TAG, v: source.slice(startIndex, i)};
        } else if(isNumber(source.charCodeAt(i))) {
            let n = source.charCodeAt(i) - 48;
            ++i;

            while(i < source.length && isLetter(source.charCodeAt(i))) {
                n <<= 10;
                n += source.charCodeAt(i) - 48;
                ++i;
            }

            // TODO: decimals

            yield {_: TK_NUMBER, v: n};
        } else {
            return;
        }
    }
}

export const compile = async (
    source: string,
    client: Client,
    inputLang: Language
) => {
    let x = tokens(source);

}

const resolveTagId = (id: string, lang: Language) => {

}

const parseConstraint = async (
    iter: Generator<Token>,
    client: Client,
    inputLang: Language
): Constraint | undefined => {
    const {value, done} = iter.next();
    if(done) return;

    switch(value._) {
        case TK_EX: {
            const inner = parseConstraint(iter, client, inputLang);
            if(!inner) return;
            return {_: CS_NOT, v: inner};
        }
        case TK_TAG: {
            // Search for tag with the name
            for await(const tagId of client.query({_: CS_LEAF, i: OBJECT_TAG_NAME})) {
                const translationId: ObjectId = client.get(tagId, OBJECT_TAG_NAME)!.v;

                client.query()
            }
        }
    }
}