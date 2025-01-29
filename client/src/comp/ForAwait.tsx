import {Accessor, createEffect, createSignal, JSX} from "solid-js";

export default <T,>({
                        iter,
                        map
}: {
    iter: Accessor<AsyncIterable<T>>,
    map: (value: T) => JSX.Element
}) => {
    const [items, setItems] = createSignal<T[]>([]);
    createEffect(async () => {
        setItems([]);

        for await(const t of iter()) {
            setItems(items => [...items, t]);
        }
    });
    return () => items().map(map);
}