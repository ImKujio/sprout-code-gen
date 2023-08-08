import artTemplate from "./art-template/index.js";
import {CamelCase, camelCase, kebabCase, snake_case} from "./nameCase.js";

function proc(tem) {
    const arr = tem.split("{{line}}")
    if (arr.length <= 1) return tem
    for (let i = 1; i < arr.length; i++) {
        const s = arr[i].split("{{/line}}")
        if (s.length !== 2) {
            console.error("{{line}} tag is not close")
            return tem
        }
        const line = s[0].replace(/(([ \t])*\n([ \t])*)+/g, " ").replace(/ {{/g, "{{").trim()
        arr.splice(i, 1, line + s[1])
    }
    return arr.join("")
}

function optimize(code) {
    ["a"].includes("")
    return code.replace(/ *\n([ \t])*\n/g, "\n").replace("{{ }}"," ")
}

export function render(tem, obj) {
    try {
        return optimize(artTemplate.render(proc(tem), obj))
    } catch (e) {
        return e.toString()
    }
}

export function initRender() {
    artTemplate.defaults.imports.snake_case = function (value) {
        return snake_case(value)
    }

    artTemplate.defaults.imports.CamelCase = function (value) {
        return CamelCase(value)
    }

    artTemplate.defaults.imports.camelCase = function (value) {
        return camelCase(value)
    }

    artTemplate.defaults.imports.kebabCase = function (value) {
        return kebabCase(value)
    }

}