import {Client} from "../adapter";
import {Accessor, createEffect, createSignal, For, Setter} from "solid-js";
import {ObjectId} from "../adapter/model.ts";
import SidePanel from "./SidePanel.tsx";
import {LANG_DEU, LANG_ENG, Language, LANGUAGE_MAP} from "../adapter/lang.ts";
import {BUILT_IN_TAG_ID_FILE} from "../adapter/objects.ts";
import Fallback from "./Fallback.tsx";
import Search from "./Search.tsx";
import {Constraint} from "../adapter/constraints.ts";

export default ({client}: {client: Client}) => {
    const [displayedObjects, setDisplayedObjects] = createSignal<ObjectId[]>([]);
    const [selected, setSelected] = createSignal<ObjectId | undefined>();
    const [language, setLanguage] = createSignal<Language>(LANG_ENG);
    const [query, setQuery] = createSignal<Constraint | undefined>(undefined);

    const populateObjects = async() => {
        setDisplayedObjects([]);

        for await(const ids of client.queryBatched(query(), 32)) {
            setDisplayedObjects(objects => [...objects, ...ids]);
        }
    };

    createEffect(async() => {
        await populateObjects();
    });

    return (
        <div
            class={"overflow-hidden h-screen flex flex-col"}
            onDragOver={e => e.preventDefault()}
            onDrop={async e => {
                e.preventDefault();

                // TODO: `Promise.all(...)` optimization possible
                for(const file of e.dataTransfer?.files || []) {
                    const blob = new Blob([await file.arrayBuffer()]);

                    const oid = await client.createObject();
                    if(!oid) return; // TODO: error handling

                    await client.associate(oid, BUILT_IN_TAG_ID_FILE, blob);
                }
            }}
        >
            <header class={"p-6 flex"}>
                <div class={"flex w-full"}>
                    <button
                        onMouseDown={() => {
                            client.createObject();
                        }}
                        class={"hover:bg-primary-600 active:bg-primary-700 cursor-pointer block bg-primary-500 text-white font-semibold text-lg px-3 rounded-full"}
                    >+ Create Object
                    </button>
                    <button
                        onMouseDown={() => {
                            populateObjects();
                        }}
                    >Refresh
                    </button>
                </div>
                <Search
                    client={client}
                    setQuery={setQuery}
                    language={language}
                />
                <div class={"flex w-full"}>
                    <button
                        onclick={() => {
                            if(language() === LANG_ENG) {
                                setLanguage(LANG_DEU);
                            } else {
                                setLanguage(LANG_ENG);
                            }
                        }}
                    >
                        Language: {LANGUAGE_MAP[language()]}
                    </button>
                </div>
            </header>
            <div class={"flex overflow-hidden h-full"}>
                <main class={"flex flex-wrap w-full overflow-y-auto gap-4 content-start p-6"}>
                    <For each={displayedObjects()}>
                        {id => <ObjectPreview
                            setSelected={setSelected}
                            client={client}
                            id={id}
                            selected={selected}
                        />}
                    </For>
                </main>
                <SidePanel
                    client={client}
                    objectId={selected}
                    language={language}
                />
            </div>
        </div>
    );
}

const ObjectPreview = ({
    id,
    client,
    setSelected,
    selected,
}: {
    id: ObjectId,
    client: Client,
    selected: Accessor<ObjectId | undefined>,
    setSelected: Setter<ObjectId | undefined>
}) => {
    return (
        <div
            class={`${selected() === id ? "bg-shade-200" : "bg-shade-100"} select-none w-32 h-32 p-4 rounded-2xl`}
            onMouseDown={() => {
                // if(selected() === id) setSelected(undefined);
                // else setSelected(id)
                setSelected(id);
            }}
        >
            <Fallback
                value={client.get(id, BUILT_IN_TAG_ID_FILE).then(x => (
                    <img
                        src={URL.createObjectURL(x as any)}
                        alt={""}
                    />
                ))}
            />
            #{id}
        </div>
    )
}