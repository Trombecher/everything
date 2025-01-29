import {createSignal} from "solid-js";
import LoginPane from "./comp/LoginPane.tsx";
import {Client} from "./adapter";
import ClientComponent from "./comp/ClientComponent.tsx";

export default () => {
    const [client, setClient] = createSignal<Client | undefined>(undefined);

    return (
        <>
            {client()
                ? <ClientComponent client={client()!}/>
                : <Home onClient={setClient}/>}
        </>
    )
}

const Home = ({
                  onClient
}: {
    onClient: (client: Client) => any
}) => {
    return (
        <div class={"flex flex-col gap-6 h-screen justify-center items-center"}>
            <LoginPane
                onLogin={async(userName, passKey) => {
                    const client = await Client.create(
                        userName,
                        new Uint8Array(passKey)
                    );

                    if(!client) return false;

                    onClient(client);

                    return true;
                }}
            />
            <div class={"max-w-sm w-full border border-shade-100 rounded-3xl p-6"}>
                <h2 class={"text-3xl font-semibold mb-6"}>Local Sandbox</h2>
                <p class={"mb-8"}>
                    This opens a <i>local</i> sandbox. If you refresh the page,
                    all changes will be lost.
                </p>
                <button
                    class={"cursor-pointer hover:bg-primary-600 active:bg-primary-700 select-none bg-primary-500 rounded-xl text-white font-semibold text-lg w-full block py-1"}
                    onClick={() => {
                        onClient(Client.local())
                    }}
                >Create</button>
            </div>
        </div>
    )
}