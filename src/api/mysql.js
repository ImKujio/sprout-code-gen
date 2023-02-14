import {invoke} from "@tauri-apps/api/tauri";
import {Store} from "./store.js";

export default class Mysql {
    constructor(url) {
        this.url = url;
    }

    static async load(url) {
        await invoke("plugin:mysql|load", {db: url})
        return new Mysql(url)
    }

    static async def(){
        let setting = await Store.Setting.all()
        let mysql = setting.mysql
        if (!mysql || !mysql.user || !mysql.psw || !mysql.addr || !mysql.db)
            throw new Error("数据库未设置")
        let url = parseUrl(mysql.user,mysql.psw,mysql.addr,mysql.db)
        await invoke("plugin:mysql|load", {db: url})
        return new Mysql(url)
    }

    async query(sql) {
        return await invoke("plugin:mysql|query", {db: this.url, sql})
    }

    async exec(sql) {
        return await invoke("plugin:mysql|exec", {db: this.url, sql})
    }
}

export function parseUrl(user,psw,addr,db) {
return `mysql://${user}:${psw}@${addr}/${db}`
}