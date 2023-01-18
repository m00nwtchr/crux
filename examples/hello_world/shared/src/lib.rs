pub mod counter;
mod hello_world;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

use crux_core::Core;
pub use crux_core::Request;

pub use counter::*;

// TODO hide this plumbing

uniffi_macros::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Core<Effect, Hello> = Core::new::<Capabilities>();
}

#[wasm_bindgen]
pub fn message(data: &[u8]) -> Vec<u8> {
    CORE.message(data)
}

#[wasm_bindgen]
pub fn response(uuid: &[u8], data: &[u8]) -> Vec<u8> {
    CORE.response(uuid, data)
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    CORE.view()
}
