use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct TutorialClickSide {
    #[wasm_bindgen(readonly)]
    pub tab: u8,
}

impl TutorialClickSide {
    #[inline]
    pub fn new(tab: u8) -> TutorialClickSide {
        return TutorialClickSide {
            tab,
        }
    }
}

impl MessageDecoder<TutorialClickSide> for TutorialClickSide {
    #[inline]
    fn length() -> i32 {
        return 1;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> TutorialClickSide {
        return TutorialClickSide::new(buf.g1());
    }
}