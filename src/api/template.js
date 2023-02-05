import artTemplate from "./art-template/index.js";

function proc(tem) {
    const arr = tem.split("{{line}}")
    if (arr.length <= 1) return tem
    for (let i = 1; i < arr.length; i++) {
        const s = arr[i].split("{{/line}}")
        if (s.length !== 2) {
            console.error("{{line}} tag is not close")
            return tem
        }
        const line = s[0].replace(/(([ \t])*\n([ \t])*)+/g, " ").replace(/ {{/g,"{{").trim()
        arr.splice(i, 1, line + s[1])
    }
    return arr.join("")
}

function optimize(code) {
    return code.replace(/ *\n([ \t])*\n/g, "\n")
}

export function render(tem, obj) {
    return optimize(artTemplate.render(proc(tem), obj))
}

export function initRender() {
    artTemplate.defaults.imports.snake_case = function (value) {
        const arr = Array.from(value)
        for (let i = 0; i < arr.length; i++) {
            if ("ABCDEFGHIJKLMNOPKRSTUVWXYZ".indexOf(arr[i]) !== -1) {
                if (i === 0) arr[i] = arr[i].toLowerCase()
                else arr[i] = "_" + arr[i].toLowerCase()
            }
        }
        return arr.join("")
    }

    artTemplate.defaults.imports.CamelCase = function (value) {
        const arr = Array.from(value)
        arr[0] = arr[0].toUpperCase()
        if (arr.length === 1) return arr.join("")
        for (let i = 1; i < arr.length; i++) {
            if (arr[i - 1] === "_") {
                arr[i - 1] = ""
                arr[i] = arr[i].toUpperCase()
            }
        }
        return arr.join("")
    }

    artTemplate.defaults.imports.camelCase = function (value) {
        const arr = Array.from(value)
        arr[0] = arr[0].toLowerCase()
        if (arr.length === 1) return arr.join("")
        for (let i = 1; i < arr.length; i++) {
            if (arr[i - 1] === "_") {
                arr[i - 1] = ""
                arr[i] = arr[i].toUpperCase()
            }
        }
        return arr.join("")
    }
}