use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IdleTimer {}

#[wasm_bindgen]
impl IdleTimer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IdleTimer {
        return IdleTimer {}
    }
}

impl MessageDecoder<IdleTimer> for IdleTimer {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> IdleTimer {
        return IdleTimer::new();
    }
}