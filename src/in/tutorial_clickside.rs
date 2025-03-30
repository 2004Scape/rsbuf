use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct TutorialClickSide {
    #[wasm_bindgen(readonly)]
    pub tab: u8,
}

#[wasm_bindgen]
impl TutorialClickSide {
    #[wasm_bindgen(constructor)]
    pub fn new(tab: u8) -> TutorialClickSide {
        return TutorialClickSide {
            tab,
        }
    }
}

impl MessageDecoder<TutorialClickSide> for TutorialClickSide {
    fn length() -> i32 {
        return 1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> TutorialClickSide {
        return TutorialClickSide::new(buf.g1());
    }
}