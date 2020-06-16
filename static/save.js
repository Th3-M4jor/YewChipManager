import {saveAs} from "file-saver";

export function save_json(data) {
    let blob = new Blob([data], {type: "application/json;charset=utf-8"});
    saveAs(blob, "pack.json");
}

export function save_txt(data) {
    let blob = new Blob([data], { type: "text/plain;charset=utf-8" });
    saveAs(blob, "pack.txt");
}