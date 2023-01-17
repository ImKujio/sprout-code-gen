import {invoke} from "@tauri-apps/api/tauri";

export async function test(msg) {
    return await invoke("test", {msg})
}