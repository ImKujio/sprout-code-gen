export function array2Tree(arr, pid) {
    const rst = []
    arr.forEach(item => {
        if (item.pid === pid) {
            const child = array2Tree(arr, item.id)
            if (child.length) item.child = child
            rst.push(item)
        }
    })
    return rst
}