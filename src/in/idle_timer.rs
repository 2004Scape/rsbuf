use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IdleTimer {}

impl IdleTimer {
    #[inline]
    pub fn new() -> IdleTimer {
        return IdleTimer {}
    }
}

impl MessageDecoder<IdleTimer> for IdleTimer {
    #[inline]
    fn length() -> i32 {
        return 0;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> IdleTimer {
        return IdleTimer::new();
    }
}