export function snake_case(value){
    const arr = Array.from(value)
    for (let i = 0; i < arr.length; i++) {
        if ("ABCDEFGHIJKLMNOPKRSTUVWXYZ".indexOf(arr[i]) !== -1) {
            if (i === 0) arr[i] = arr[i].toLowerCase()
            else arr[i] = "_" + arr[i].toLowerCase()
        }
    }
    return arr.join("")
}

export function CamelCase(value) {
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

export function camelCase(value){
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

export function kebabCase(value){
    const arr = Array.from(value)
    for (let i = 0; i < arr.length; i++) {
        if ("ABCDEFGHIJKLMNOPKRSTUVWXYZ".indexOf(arr[i]) !== -1) {
            if (i === 0) arr[i] = arr[i].toLowerCase()
            else arr[i] = "-" + arr[i].toLowerCase()
        }
    }
    return arr.join("")
}