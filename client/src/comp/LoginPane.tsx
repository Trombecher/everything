import {createEffect, createSignal} from "solid-js";
import {LocalStore} from "../localStore.ts";

const ERR_PROVIDE_KEY = "Please provide a pass key.";
const ERR_INVALID_CREDENTIALS = "Invalid user name or pass key.";

type Error = typeof ERR_PROVIDE_KEY | typeof ERR_INVALID_CREDENTIALS;

export default ({
                    onLogin,
                }: {
    onLogin: (userName: string, passKey: ArrayBuffer) => Promise<boolean>
}) => {
    const [userName, setUserName] = createSignal(LocalStore.getLastUsedUserName() || "");
    const [passKey, _setPassKey] = createSignal<ArrayBuffer | undefined>(undefined);

    const [loggingIn, setLoggingIn] = createSignal<boolean>(false);
    const [error, setError] = createSignal<Error | undefined>(undefined);

    createEffect(() => {
        LocalStore.setLastUsedUserName(userName());
    });

    return (
        <div class={"max-w-sm w-full bg-white rounded-3xl p-6"}>
            <h2 class={"text-3xl font-semibold mb-6"}>Login</h2>
            {error()}
            <label class={"block"}>
                User name:
                <input
                    value={userName()}
                    type="text"
                    onInput={e => setUserName(e.target.value)}
                    class={"block"}
                />
            </label>
            <label class={"block"}>
                Passkey:
                <input
                    class={"block"}
                    type="file"
                />
            </label>
            <button
                class={"block"}
                disabled={loggingIn()}
                onClick={() => {
                    const pk = passKey();

                    if(!pk) {
                        setError(ERR_PROVIDE_KEY);
                        return;
                    }

                    setLoggingIn(true);

                    onLogin(userName(), pk).then(ok => {
                        setLoggingIn(false);

                        if(!ok) setError(ERR_INVALID_CREDENTIALS);
                    });
                }}
            >Login
            </button>
        </div>
    );
}