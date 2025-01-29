export namespace LocalStore {
    export const getLastUsedUserName = () => localStorage.lastUsedUserName;
    export const setLastUsedUserName = (userName: string) => {
        localStorage.lastUsedUserName = userName;
    };
}