use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CloseModal {}

impl CloseModal {
    const DEFAULT: CloseModal = CloseModal::new();

    #[inline]
    pub const fn new() -> CloseModal {
        return CloseModal {}
    }
}

impl MessageDecoder<CloseModal> for CloseModal {
    #[inline]
    fn length() -> i32 {
        return 0;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> CloseModal {
        return CloseModal::DEFAULT;
    }
}