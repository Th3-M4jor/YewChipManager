use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub(crate) enum Elements {
    Fire,
    Aqua,
    Elec,
    Wood,
    Wind,
    Sword,
    Break,
    Cursor,
    Recovery,
    Invis,
    Object,
    Null,
}


//encoded each png in base64 since they are rather small
/*
const FIRE_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOBAMAAADtZjDiAAAABGdBTUEAALGPC/xhBQAAABVQTFRFKCgoeHB4oDBIsKio6DBI+JgA////esoD/wAAAE1JREFUCB0FwQENwjAUBcDrWwWM1MDPHIAlDCNgWeaAKijctfcHNk+okZ9IzRAHoVJ0KYhDiaQQifZA2O1kwZJ1T/PS26nOe/RXfMfY/nf4D6En5pTGAAAAAElFTkSuQmCC";
const AQUA_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAFRJREFUKJFj+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlGLKRTT8QBPBrhSojiilEHWYqilTSqwDMFXARVCUYhqGLIJQiulENBGylBLrAGIjlvjkAgAWUOd9VSCMVAAAAABJRU5ErkJggg==";
const ELEC_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOBAMAAADtZjDiAAAABGdBTUEAALGPC/xhBQAAABJQTFRFKCgoeHB4sKio+JgA+PAo////RbdwrwAAAEZJREFUCB0VwQENhEAMBMBJ+wpQsJDg4AVcSBXgXwy5Ge+13U56OUQPJT2LktkeoWdK6Fkl/EYJ/6WEoYRFiUY5bYf32u4PMa4I9zsAdFQAAAAASUVORK5CYII=";
const WOOD_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOBAMAAADtZjDiAAAABGdBTUEAALGPC/xhBQAAAA9QTFRFKCgoMMhYeHB4sKio////UNJopAAAAERJREFUCNc1zbERwDAIxVBxLODPBjgLcCEbZP+dUuBUr5N4u7v74QbgosC0ncIUcgopx9CYWk4RGlPCKQNwah0Z/+75fLv0B4IMG42/AAAAAElFTkSuQmCC";
const WIND_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOBAMAAADtZjDiAAAABGdBTUEAALGPC/xhBQAAAA9QTFRFKCgoeHB4sKio6Oj4////wq8ncQAAAEZJREFUCNc9jcENhAAMw6yyARPkUgZAzQbsPxSfHq9Iluzw2LYvfgCciI4odCShUJKocKZzF52e3eWd6UEZ96yn+jr/7v68j7ELsbTr2fEAAAAASUVORK5CYII=";
const SWORD_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOBAMAAADtZjDiAAAABGdBTUEAALGPC/xhBQAAAA9QTFRFAOD4KCgoeHB4sKio////lIUzwwAAAD1JREFUCB0FwQERwwAIBLA8hwA6S/OvYRKKA5bku5B+f/Dpg7cKjoKNwhmFDe0YSjYoZ1A2i2ag8sD0E8gf39UOUJ0nXL4AAAAASUVORK5CYII=";
const BREAK_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAFFJREFUKM9j+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlGoQAbAoBRoAR/iUIqtDU42iFCL34sUPIMJUTZlSXG4gVykJ3iItsAhHAbERS3xyAQCqnefV7r0a1gAAAABJRU5ErkJggg==";
const CURSOR_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAEtJREFUKM9j+P///4YVKwgioDIGIFVRUaGBFwAVAJWBlEL4HzoYIAgrF6EULoGpFCKCrhTTasqUEgRkKaWNW0kILGKjgNiIJT65AACsmN81zVCWHQAAAABJRU5ErkJggg==";
const RECOVERY_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAGJJREFUKM9j+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlGoQAXAqfWHgAUTIDOxKIdJoCKH0R8YKIIIzFuBSCpFGQwvIU4pwK36l6CGApg7uP+yBhaaOQLjiUodQSmzEEp9cAE4xAlRqvKO9AAAAAElFTkSuQmCC";
const INVIS_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAFFJREFUKJFj+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlKJJrGD4AURoguhK0VQgc7GbihWgKMW0FFmQXFNp41Y0k9BsoKm3IBGLx1RoxBKfXACrMsNFvVcK4wAAAABJRU5ErkJggg==";
const OBJECT_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAEpJREFUKM9j+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlGoQAdCVHsiwwERYlGJVt8DAA4vSHzMY0BBQ3TBVijNcCUcBsRFLfHIBADJ27qX4xuXwAAAAAElFTkSuQmCC";
const NULL_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAIAAACQKrqGAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAAAgY0hSTQAAeiYAAICEAAD6AAAAgOgAAHUwAADqYAAAOpgAABdwnLpRPAAAADVJREFUKM9j+P///4YVKwgioDIGIFVRUKGBFwAVAJWBlGoQAQapUoiXqa10UIcAsRFLfHIBANLVuTXoUOn7AAAAAElFTkSuQmCC";
*/

impl Elements {
    /*
    pub(crate) fn to_img_url(&self) -> &'static str {
        match self {
            Elements::Fire => FIRE_URL,
            Elements::Aqua => AQUA_URL,
            Elements::Elec => ELEC_URL,
            Elements::Wood => WOOD_URL,
            Elements::Wind => WIND_URL,
            Elements::Sword => SWORD_URL,
            Elements::Break => BREAK_URL,
            Elements::Cursor => CURSOR_URL,
            Elements::Recovery => RECOVERY_URL,
            Elements::Invis => INVIS_URL,
            Elements::Object => OBJECT_URL,
            Elements::Null => NULL_URL,
        }
    }
    */

    pub(crate) fn to_css_class(&self) -> &'static str {
        match self {
            Elements::Fire => "fireChip",
            Elements::Aqua => "aquaChip",
            Elements::Elec => "elecChip",
            Elements::Wood => "woodChip",
            Elements::Wind => "windChip",
            Elements::Sword => "swordChip",
            Elements::Break => "breakChip",
            Elements::Cursor => "cursorChip",
            Elements::Recovery => "recovChip",
            Elements::Invis => "invisChip",
            Elements::Object => "objectChip",
            Elements::Null => "nullChip",
        }
    }

    // fn to intern urls in JS as copying from rust is expensive
    /*
    pub(crate) fn intern_urls() {
        wasm_bindgen::intern(FIRE_URL);
        wasm_bindgen::intern(AQUA_URL);
        wasm_bindgen::intern(ELEC_URL);
        wasm_bindgen::intern(WOOD_URL);
        wasm_bindgen::intern(WIND_URL);
        wasm_bindgen::intern(SWORD_URL);
        wasm_bindgen::intern(BREAK_URL);
        wasm_bindgen::intern(CURSOR_URL);
        wasm_bindgen::intern(RECOVERY_URL);
        wasm_bindgen::intern(INVIS_URL);
        wasm_bindgen::intern(OBJECT_URL);
        wasm_bindgen::intern(NULL_URL);
    }
    */
}