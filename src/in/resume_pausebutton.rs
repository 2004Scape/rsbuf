use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ResumePauseButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

impl ResumePauseButton {
    #[inline]
    pub fn new(component: u16) -> ResumePauseButton {
        return ResumePauseButton {
            component,
        };
    }
}

impl MessageDecoder<ResumePauseButton> for ResumePauseButton {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> ResumePauseButton {
        return ResumePauseButton::new(buf.g2());
    }
}