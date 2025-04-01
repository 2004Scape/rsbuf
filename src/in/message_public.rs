use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MessagePublic {
    #[wasm_bindgen(readonly)]
    pub color: u8,
    #[wasm_bindgen(readonly)]
    pub effect: u8,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: Vec<u8>,
}

impl MessagePublic {
    #[inline]
    pub fn new(
        color: u8,
        effect: u8,
        input: Vec<u8>,
    ) -> MessagePublic {
        return MessagePublic {
            color,
            effect,
            input,
        }
    }
}

impl MessageDecoder<MessagePublic> for MessagePublic {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> MessagePublic {
        return MessagePublic::new(
            buf.g1(),
            buf.g1(),
            unsafe { buf.data.get_unchecked(buf.pos..buf.pos + buf.data.len() - 2).to_vec() }
        );
    }
}