import { run } from '../Cargo.toml'

const URL = "https://spartan364.hopto.org/chips.json";

async function start() {
    
    try {
        let body = await fetch(URL);
        let result = await body.text();
        run(result);

    } catch (_) {
        alert("an error occurred in loading chips, inform Major");
    }
}
start();
//run()