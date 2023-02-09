import {Store as KVStore} from "tauri-plugin-store-api";

KVStore.prototype.list = function () {
    return this.get("list")
}
KVStore.prototype.setList = function (val) {
    return this.set("list", val)
}

KVStore.prototype.info = function (id) {
    return this.get("info/" + id)
}
KVStore.prototype.setInfo = function (id, val) {
    return this.set("info/" + id, val)
}

KVStore.prototype.all = function () {
    return this.get("all")
}
KVStore.prototype.put = function (val) {
    return this.set("all", val)
}

export const Store = {
    Classes: new KVStore("classes.json"),
    Templates: new KVStore("templates.json"),
    PropColumns: new KVStore("propcolumns.json"),
    ClassColumns: new KVStore("classcolumns.json"),
    Setting: new KVStore("setting.json")
}