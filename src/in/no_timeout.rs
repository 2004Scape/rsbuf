use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct NoTimeout {}

#[wasm_bindgen]
impl NoTimeout {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NoTimeout {
        return NoTimeout {};
    }
}

impl MessageDecoder<NoTimeout> for NoTimeout {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> NoTimeout {
        return NoTimeout::new();
    }
}