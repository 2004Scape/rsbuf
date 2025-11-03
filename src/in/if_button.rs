use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IfButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

impl IfButton {
    #[inline]
    pub fn new(component: u16) -> IfButton {
        return IfButton {
            component,
        }
    }
}

impl MessageDecoder<IfButton> for IfButton {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> IfButton {
        return IfButton::new(buf.g2());
    }
}