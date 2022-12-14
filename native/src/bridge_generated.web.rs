use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_platform(port_: MessagePort) {
    wire_platform_impl(port_)
}

#[wasm_bindgen]
pub fn wire_rust_release_mode(port_: MessagePort) {
    wire_rust_release_mode_impl(port_)
}

#[wasm_bindgen]
pub fn wire_hello_world(port_: MessagePort) {
    wire_hello_world_impl(port_)
}

// Section: allocate functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue
