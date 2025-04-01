use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MessagePrivate {
    #[wasm_bindgen(readonly)]
    pub username: i64,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: Vec<u8>,
}

impl MessagePrivate {
    #[inline]
    pub fn new(
        username: i64,
        input: Vec<u8>,
    ) -> MessagePrivate {
        return MessagePrivate {
            username,
            input,
        }
    }
}

impl MessageDecoder<MessagePrivate> for MessagePrivate {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> MessagePrivate {
        return MessagePrivate::new(
            buf.g8s(),
            unsafe { buf.data.get_unchecked(buf.pos..buf.pos + buf.data.len() - 8).to_vec() }
        );
    }
}