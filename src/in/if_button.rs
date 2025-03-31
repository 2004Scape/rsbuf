use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IfButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl IfButton {
    #[wasm_bindgen(constructor)]
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
    fn decode(_: ClientProt, buf: &mut Packet) -> IfButton {
        return IfButton::new(buf.g2());
    }
}