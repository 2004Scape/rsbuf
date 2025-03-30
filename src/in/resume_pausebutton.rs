use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ResumePauseButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl ResumePauseButton {
    pub fn new(component: u16) -> ResumePauseButton {
        return ResumePauseButton {
            component,
        };
    }
}

impl MessageDecoder<ResumePauseButton> for ResumePauseButton {
    fn length() -> i32 {
        return 2;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ResumePauseButton {
        return ResumePauseButton::new(buf.g2());
    }
}