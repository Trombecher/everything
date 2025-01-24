import {Adapter__close, Adapter__new, Adapter__objects__iter} from "../../client/src/adapter/adapter.ts";

const adapter = await Adapter__new("admin", new TextEncoder().encode("password"));

for await (const object of Adapter__objects__iter(adapter)) {
    console.log("Received object:", object);
}

process.on("SIGINT", async () => {
    await Adapter__close(adapter);
});