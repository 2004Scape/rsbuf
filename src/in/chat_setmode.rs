use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ChatSetMode {
    #[wasm_bindgen(readonly)]
    pub public: u8,
    #[wasm_bindgen(readonly)]
    pub private: u8,
    #[wasm_bindgen(readonly)]
    pub trade: u8,
}

impl ChatSetMode {
    #[inline]
    pub fn new(
        public: u8,
        private: u8,
        trade: u8,
    ) -> ChatSetMode {
        return ChatSetMode {
            public,
            private,
            trade,
        }
    }
}

impl MessageDecoder<ChatSetMode> for ChatSetMode {
    #[inline]
    fn length() -> i32 {
        return 3;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> ChatSetMode {
        return ChatSetMode::new(
            buf.g1(),
            buf.g1(),
            buf.g1(),
        );
    }
}