import {invoke} from "@tauri-apps/api/tauri";


function getData(key, raw) {
    return new Promise((resolve, reject) => {
        invoke("get_data", {key: key}).then(rst => {
            console.log("getData:" + key, rst)
            if (rst.ok) {
                !raw ? resolve(JSON.parse(rst.data)) : resolve(rst.data)
            } else {
                reject(rst.msg)
            }
        })
    })
}

function setData(key, val, raw) {
    return new Promise((resolve, reject) => {
        invoke("set_data", {key: key, val: !raw ? JSON.stringify(val) : val}).then(rst => {
            console.log("setData:" + key, {val, rst})
            if (rst.ok) {
                resolve()
            } else {
                reject(rst.msg)
            }
        })
    })
}

function toJson(val) {
    return JSON.stringify(val)
}

function toObj(val) {
    return JSON.parse(val)
}

export const Template = {
    Spring: {
        get() {
            return getData("Template/Spring",true)
        },
        set(val) {
            return setData("Template/Spring", val,true)
        }
    }
}

export const Mysql = {
    Create: {
        get() {
            return getData("Mysql/Create")
        },
        set(val) {
            return setData("Mysql/Create", val)
        }
    }
}

export const Option = {
    DefColumn: {
        get() {
            return getData("Options/DefColumn")
        },
        set(val) {
            return setData("Options/DefColumn", val)
        }
    },
    DataType: {
        get() {
            return getData("Options/DataType")
        },
        set(val) {
            return setData("Options/DataType", val)
        }
    },
}