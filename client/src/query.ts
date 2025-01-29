import {Client} from "./adapter";
import {Constraint, CS_AND, CS_LEAF, CS_NOT} from "./adapter/constraints.ts";
import {ObjectId, VL_OBJECT_ID} from "./adapter/model.ts";
import {Language} from "./adapter/lang.ts";
import {
    BUILT_IN_TAG_ID_TAG_NAME,
    BUILT_IN_TAG_ID_TAG_PARENT,
    BUILT_IN_TAG_ID_TAG_TRANSLATION_LANGUAGE,
    BUILT_IN_TAG_ID_TAG_TRANSLATION_NAME,
} from "./adapter/objects.ts";
import {Accessor, createEffect, createSignal, Setter} from "solid-js";

export type Span<T> = {
    v: T,
    start: number,
    end: number
};

export const TK_TAG = 1;
export const TK_LPAREN = 2;
export const TK_RPAREN = 3;
export const TK_STRING = 4;
export const TK_PLUS = 5; // +
export const TK_PIPE = 6; // |
export const TK_EX = 7; // !
export const TK_NUMBER = 8;

export type Token = {_: typeof TK_TAG, v: string}
    | {_: typeof TK_LPAREN}
    | {_: typeof TK_RPAREN}
    | {_: typeof TK_STRING, v: string}
    | {_: typeof TK_PLUS}
    | {_: typeof TK_PIPE}
    | {_: typeof TK_EX}
    | {_: typeof TK_NUMBER, v: number};

const isLetter = (n: number) => (65 <= n && n <= 90) || (97 <= n && n <= 122) || n === 95;
const isWS = (n: number) => n === 32 || n === 10 || n === 9 || n === 11;
const isNumber = (n: number) => 48 <= n && n <= 57;

function* tokens(source: string): Generator<Span<Token>> {
    let i = 0;

    while(i < source.length) {
        // Skip whitespace
        while(isWS(source.charCodeAt(i))) {
            ++i;
            if(i >= source.length) return;
        }

        let token: Token;
        let start = i;

        if(source[i] === "!") {
            ++i;
            token = {_: TK_EX};
        } else if(source[i] === "+") {
            ++i;
            token = {_: TK_PLUS};
        } else if(source[i] === "|") {
            ++i;
            token = {_: TK_PIPE};
        } else if(source[i] === "(") {
            ++i;
            token = {_: TK_LPAREN};
        } else if(source[i] === ")") {
            ++i;
            token = {_: TK_RPAREN};
        } else if(isLetter(source.charCodeAt(i))) {
            const startIndex = i;
            ++i;

            // Skip letters
            while(i < source.length && isLetter(source.charCodeAt(i))) ++i;

            token = {_: TK_TAG, v: source.slice(startIndex, i)};
        } else if(isNumber(source.charCodeAt(i))) {
            let n = source.charCodeAt(i) - 48;
            ++i;

            while(i < source.length && isNumber(source.charCodeAt(i))) {
                n *= 10;
                n += source.charCodeAt(i) - 48;
                ++i;
            }

            // TODO: decimals

            token = {_: TK_NUMBER, v: n};
        } else if(source[i] === "\"") {
            ++i;

            while(i < source.length && source[i] !== "\"")
                ++i;

            ++i;

            token = {_: TK_STRING, v: source.slice(start + 1, i - 1)};
        } else return;

        yield {v: token, start, end: i};
    }
}

export const addWhitespaceTokens = (tokens: Span<Token>[]) => {
    const newTokens: Span<Token | undefined>[] = [];
    let i = 0;

    for(const token of tokens) {
        if(i !== token.start) {
            newTokens.push({
                v: undefined,
                start: i,
                end: token.start,
            });
        }

        newTokens.push(token);

        i = token.end;
    }

    return newTokens;
}

const resolveTagId = async (
    name: string,
    lang: Language,
    client: Client,
    parentId: ObjectId | undefined
): Promise<ObjectId | undefined> => {
    // TODO: maybe future support for advanced queries, like:
    //     `Tag.Name = $someObject and Tag.Parent = <PARENT> where $someObject: Tag.Translation.Language = <LANG> and Tag.Translation.Name = <NAME>`

    // First query: `Tag.Name` (and `Tag.Parent`)
    for await(const tagId of parentId
        ? client.query({_: CS_AND, l: {_: CS_LEAF, i: BUILT_IN_TAG_ID_TAG_NAME}, r: {_: CS_LEAF, i: BUILT_IN_TAG_ID_TAG_PARENT, v: {_: VL_OBJECT_ID, v: parentId}}})
        : client.query({_: CS_LEAF, i: BUILT_IN_TAG_ID_TAG_NAME})) {
        // We know that this is an `ObjectId`
        const translationId = (await client.get(tagId, BUILT_IN_TAG_ID_TAG_NAME))! as ObjectId;

        if(await client.get(translationId, BUILT_IN_TAG_ID_TAG_TRANSLATION_LANGUAGE) === lang && await client.get(translationId, BUILT_IN_TAG_ID_TAG_TRANSLATION_NAME) === name)
            return tagId;
    }
};

const parseConstraint = async(
    iter: Iterator<Token>,
    client: Client,
    inputLang: Language,
): Promise<Constraint | undefined> => {
    const {value, done} = iter.next();
    if(done) return;

    switch(value._) {
        case TK_EX: {
            const inner = await parseConstraint(iter, client, inputLang);
            if(!inner) return;
            return {_: CS_NOT, i: inner};
        }
        case TK_TAG: {
            const tagId = await resolveTagId(
                value.v,
                inputLang,
                client,
                undefined
            );

            if(!tagId) return;


        }
    }
};

export const compileQuery = (
    queryText: Accessor<string>,
    client: Client,
    language: Accessor<Language>,
    setQuery: Setter<Constraint | undefined>,
) => {
    const [getTokens, setTokens] = createSignal<Span<Token>[]>([]);

    createEffect(async() => {
        const collectedTokens = Array.from(tokens(queryText()));
        setTokens(collectedTokens);

        if(collectedTokens.length === 0) {
            setQuery(undefined);
            return;
        }

        // console.log(debugStringifyTokens(collectedTokens));

        /*
        const tokensIter = collectedTokens[Symbol.iterator]();

        const res = await parseConstraint(
            tokensIter,
            client,
            language()
        );

        if(!res) return;


        setQuery(res);
         */
    });

    return getTokens;
};

/*
export const debugStringifyTokens = (tokens: Token[]) => tokens.map(token => {
    switch(token._) {
        case TK_EX:
            return "!";
        case TK_PIPE:
            return "|";
        case TK_LPAREN:
            return "(";
        case TK_TAG:
            return `\`${token.v}\``;
        case TK_NUMBER:
            return `NUM ${token.v}`;
        case TK_RPAREN:
            return ")";
        case TK_STRING:
            return `"${token.v}"`;
        case TK_PLUS:
            return "+";
    }
}).join(" ");
 */