use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct InvButton {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl InvButton {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        op: u8,
        obj: u16,
        slot: u16,
        component: u16,
    ) -> InvButton {
        return InvButton {
            op,
            obj,
            slot,
            component,
        }
    }
}

impl MessageDecoder<InvButton> for InvButton {
    #[inline]
    fn length() -> i32 {
        return 6;
    }

    #[inline]
    fn decode(prot: ClientProt, mut buf: Packet) -> InvButton {
        let op: u8 = match prot {
            ClientProt::INV_BUTTON1 => 1,
            ClientProt::INV_BUTTON2 => 2,
            ClientProt::INV_BUTTON3 => 3,
            ClientProt::INV_BUTTON4 => 4,
            ClientProt::INV_BUTTON5 => 5,
            _ => 0,
        };
        return InvButton::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct InvButtonD {
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub target: u16,
}

#[wasm_bindgen]
impl InvButtonD {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        component: u16,
        slot: u16,
        target: u16,
    ) -> InvButtonD {
        return InvButtonD {
            component,
            slot,
            target,
        }
    }
}

impl MessageDecoder<InvButtonD> for InvButtonD {
    #[inline]
    fn length() -> i32 {
        return 6;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> InvButtonD {
        return InvButtonD::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}