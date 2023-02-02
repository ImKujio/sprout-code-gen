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
    Mysql: {
        Create: {
            get() {
                return getData("Mysql/Create",true)
            },
            set(val) {
                return setData("Mysql/Create", val,true)
            }
        }
    },
    Spring: {
        Entity: {
            get() {
                return getData("Spring/Entity",true)
            },
            set(val) {
                return setData("Spring/Entity", val,true)
            }
        },
        Mapper: {
            get() {
                return getData("Spring/Mapper",true)
            },
            set(val) {
                return setData("Spring/Mapper", val,true)
            }
        },
        MapperXml: {
            get() {
                return getData("Spring/MapperXml",true)
            },
            set(val) {
                return setData("Spring/MapperXml", val,true)
            }
        },
        Service: {
            get() {
                return getData("Spring/Service",true)
            },
            set(val) {
                return setData("Spring/Service", val,true)
            }
        },
        ServiceImpl: {
            get() {
                return getData("Spring/ServiceImpl",true)
            },
            set(val) {
                return setData("Spring/ServiceImpl", val,true)
            }
        },
        Controller: {
            get() {
                return getData("Spring/Controller",true)
            },
            set(val) {
                return setData("Spring/Controller", val,true)
            }
        },
    },
    Page:{
        Api: {
            get() {
                return getData("Page/Api",true)
            },
            set(val) {
                return setData("Page/Api", val,true)
            }
        },
        Component: {
            get() {
                return getData("Page/Component",true)
            },
            set(val) {
                return setData("Page/Component", val,true)
            }
        },
    },
    Option: {
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
}