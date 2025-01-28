import {Accessor, createSignal, Setter} from "solid-js";

export default () => {
    const [query, setQuery] = createSignal("");

    return (
        <>
            <QueryComponent query={query} setQuery={setQuery}/>
        </>
    )
}

const QueryComponent = ({
    query,
    setQuery,
}: {
    query: Accessor<string>,
    setQuery: Setter<string>
}) => {
    return (
        <div
            onInput={e => setQuery(e.target.value)}
            class={"bg-shade-100 text-xl rounded-full px-4 py-1 mt-4 mx-auto"}
        ></div>
    )
}