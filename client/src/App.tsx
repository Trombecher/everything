import {Adapter__new, Adapter__objects__iter} from "./adapter/adapter.ts";
import {createSignal} from "solid-js";
import {Object} from "./adapter/model.ts";

export default () => {
    const [objects, setObjects] = createSignal<Object[]>([]);

    return (
        <div>
            <button
                onclick={() => {
                    Adapter__new("admin", new TextEncoder().encode("password"))
                        .then(async adapter => {
                            for await(const object of Adapter__objects__iter(adapter)) {
                                setObjects([...objects(), object]);
                            }
                        })
                }}
            >
                click
            </button>
            {objects().map(object => (
                <div>{object.id} {object.type}</div>
            ))}
        </div>
    )
}