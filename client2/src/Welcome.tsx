export default () => {
    return (
        <div class={"flex flex-col items-center"}>
            <p class={"mt-12 text-xl mb-2 text-shade-800"}>Welcome to</p>
            <h1 class={"font-bold text-5xl tracking-tighter mb-8"}>Everything</h1>
            <button class={"bg-primary/50 px-4 font-semibold py-1 rounded-xl mb-4"}>Get started locally</button>
            <button class={"px-4 font-semibold py-1 rounded-xl bg-shade-100 border border-shade-200"}>
                Connect to Server
            </button>
        </div>
    )
}