function saveAs(blob, name) {

    let a = document.createElement('a');
    a.download = name;
    a.rel = 'noopener';
    a.href = URL.createObjectURL(blob);
    setTimeout(() => {URL.revokeObjectURL(a.href)}, 4E4); // 40s
    setTimeout(() => {a.click()});

}

export function save_json(data) {
    let blob = new Blob([data], {type: "application/json;charset=utf-8"});
    saveAs(blob, "pack.json");
}

export function save_txt(data) {
    let blob = new Blob([data], { type: "text/plain;charset=utf-8" });
    saveAs(blob, "pack.txt");
}

export function storage_available(type) {
    let storage;
    try {
        storage = window[type];
        let x = '__storage_test__';
        storage.setItem(x, x);
        storage.removeItem(x);
        return true;
    }
    catch (e) {
        return e instanceof DOMException && (
            // everything except Firefox
            e.code === 22 ||
            // Firefox
            e.code === 1014 ||
            // test name field too, because code might not be present
            // everything except Firefox
            e.name === 'QuotaExceededError' ||
            // Firefox
            e.name === 'NS_ERROR_DOM_QUOTA_REACHED') &&
            // acknowledge QuotaExceededError only if there's something already stored
            (storage && storage.length !== 0);
    }
}