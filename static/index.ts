import { run, save_before_exit } from '../Cargo.toml'

const URL = "https://spartan364.hopto.org/chips.json";

async function start() {
    
    try {
        //let body = await fetch(URL);
        //let result = await body.text();
        /*
        window.addEventListener("beforeunload", function (e) {
            let confirmationMessage = 'Progress might be lost if you leave without saving an export.';

            (e || window.event).returnValue = confirmationMessage; //Gecko + IE
            save_before_exit();
            return confirmationMessage; //Gecko + Webkit, Safari, Chrome etc.
        });
        */
        run();

    } catch (_) {
        alert("an error occurred in loading chips, inform Major");
    }


}
start();
//run()