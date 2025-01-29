import {Client} from "../adapter";
import {Accessor} from "solid-js";
import {ObjectId} from "../adapter/model.ts";
import ForAwait from "./ForAwait.tsx";
import {Language} from "../adapter/lang.ts";
import ValueComponent from "./ValueComponent.tsx";

export default ({
                    objectId,
                    client,
                    language,
                }: {
    client: Client,
    objectId: Accessor<ObjectId | undefined>,
    language: Accessor<Language>
}) => {
    return (
        <div class={`${objectId() ? "" : "hidden"} w-full overflow-y-auto`}>
            {objectId()}

            <ForAwait
                iter={async function* () {
                    if(!objectId()) return;
                    for await (const i of client.tags(objectId()!)) {
                        yield [
                            (await client.getTagName(i, language()))!,
                            (await client.get(objectId()!, i)),
                        ] as const;
                    }
                }}
                map={([objectId, value]) => (
                    <div>
                        {objectId}
                        {value && <>
                            {" ="} <ValueComponent value={value}/>
                        </>}
                    </div>
                )}
            />
        </div>
    );
}