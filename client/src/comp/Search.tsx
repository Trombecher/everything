import {Accessor, createSignal, For, Setter} from "solid-js";
import {Constraint} from "../adapter/constraints.ts";
import {Client} from "../adapter";
import {
    addWhitespaceTokens,
    compileQuery,
    Span,
    TK_EX, TK_LPAREN, TK_NUMBER,
    TK_PIPE, TK_PLUS,
    TK_RPAREN,
    TK_STRING,
    TK_TAG,
    Token,
} from "../query.ts";
import {Language} from "../adapter/lang.ts";

export default ({
                    setQuery,
                    client,
                    language,
                }: {
    setQuery: Setter<Constraint | undefined>,
    client: Client,
    language: Accessor<Language>,
}) => {
    const [queryText, setQueryText] = createSignal("");
    const tokens = compileQuery(
        queryText,
        client,
        language,
        setQuery,
    );

    return (
        <label
            class={"items-center flex py-1 px-1 transition shrink-0 w-full max-w-md rounded-3xl bg-shade-100 focus-within:bg-shade-200"}>
            <svg class={"shrink-0 fill-shade-900"} xmlns={"http://www.w3.org/2000/svg"} width={"32"} height={"32"}
                 viewBox={"0 0 32 32"}>
                <path
                    d={"M19 19A1 1 0 0010 10 1 1 0 0019 19M18 18A1 1 0 0111 11 1 1 0 0118 18M25 25 19 18 18 19 24 26Z"}/>
            </svg>
            <input
                class={"w-full text-lg font-mono pl-1 pr-3 outline-none"}
                type="text"
                spellcheck={false}
                onInput={e => setQueryText(e.target.value)}
                value={queryText()}
            />
        </label>
    );
}

const Tokens = ({
                    tokens,
                }: {
    tokens: Accessor<Span<Token>[]>
}) => {
    return (
        <For each={addWhitespaceTokens(tokens())}>
            {token => {
                if(!token.v) return <span>{"\u00A0".repeat(token.end - token.start)}</span>;

                switch(token.v?._) {
                    case TK_STRING:
                        return <span class={"text-green"}>"{token.v.v}"</span>;
                    case TK_TAG:
                        return <i>{token.v.v}</i>;
                    case TK_PIPE:
                        return <span>|</span>;
                    case TK_EX:
                        return <span>!</span>;
                    case TK_LPAREN:
                        return <span>(</span>;
                    case TK_RPAREN:
                        return <span>)</span>;
                    case TK_NUMBER:
                        return <span class={"text-blue"}>{token.v.v}</span>;
                    case TK_PLUS:
                        return <span>+</span>;
                }
            }}
        </For>
    );
};