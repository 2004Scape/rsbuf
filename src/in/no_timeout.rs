use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct NoTimeout {}

impl NoTimeout {
    const DEFAULT: NoTimeout = NoTimeout::new();

    #[inline]
    pub const fn new() -> NoTimeout {
        return NoTimeout {};
    }
}

impl MessageDecoder<NoTimeout> for NoTimeout {
    #[inline]
    fn length() -> i32 {
        return 0;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> NoTimeout {
        return NoTimeout::DEFAULT;
    }
}