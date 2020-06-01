use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[wasm_bindgen]
pub struct TimeoutHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for TimeoutHandle {
    fn drop(&mut self) {
        let window = web_sys::window().unwrap();
        window.clear_timeout_with_handle(self.interval_id);
    }
}

pub fn set_timeout<F: FnMut() + 'static>(interval: i32, callback: F) -> Result<TimeoutHandle, JsValue> {
    let window = web_sys::window().unwrap();
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

    let id = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        interval,
    )?;

    Ok(TimeoutHandle {
        interval_id: id,
        _closure: closure,
    })

}