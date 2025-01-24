import {defineConfig} from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
    plugins: [solid(), tailwindcss()],
    server: {
        proxy: {
            "/connect": {
                target: "ws://localhost:3000",
                ws: true,
            },
        }
    }
});