import {invoke} from "@tauri-apps/api/tauri";

export default class Mysql {
    constructor(url) {
        this.url = url;
    }

    static async load(url) {
        await invoke("plugin:mysql|load", {db: url})
        return new Mysql(url)
    }

    async query(q) {
        return await invoke("plugin:mysql|query", {db: this.url, query: q})
    }
}