use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CloseModal {}

#[wasm_bindgen]
impl CloseModal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CloseModal {
        return CloseModal {}
    }
}

impl MessageDecoder<CloseModal> for CloseModal {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> CloseModal {
        return CloseModal::new();
    }
}