import {invoke} from "@tauri-apps/api/tauri";

function getData(key) {
    return new Promise((resolve, reject) => {
        invoke("get_data", {key: key}).then(rst => {
            console.log("getData:"+key,rst)
            if (rst.ok) {
                resolve(JSON.parse(rst.data))
            } else {
                reject(rst.msg)
            }
        })
    })
}

function setData(key, val) {
    return new Promise((resolve, reject) => {
        invoke("set_data", {key: key, val: JSON.stringify(val)}).then(rst => {
            console.log("setData:"+key, {val, rst})
            if (rst.ok) {
                resolve()
            } else {
                reject(rst.msg)
            }
        })
    })
}

export const Template = {
    Spring:{
        get() {
            return getData("Template/Spring")
        },
        set(val) {
            return setData("Template/Spring", val)
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