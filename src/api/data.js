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

export const Data = {
    Modules:{
        get() {
            return getData("Modules")
        },
        set(val) {
            return setData("Modules", val)
        },
        getInfo(id) {
            return getData("Modules/"+id)
        },
        setInfo(id,val){
            return setData("Modules/"+id,val)
        }
    },
    Templates:{
        get() {
            return getData("Templates")
        },
        set(val) {
            return setData("Templates", val)
        },
        getInfo(id) {
            return getData("Templates/"+id)
        },
        setInfo(id,val){
            return setData("Templates/"+id,val)
        }
    },
    PropTypes:{
        get() {
            return getData("PropTypes")
        },
        set(val) {
            return setData("PropTypes", val)
        }
    },

}