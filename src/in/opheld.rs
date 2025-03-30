use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct OpHeld {
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
impl OpHeld {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        obj: u16,
        slot: u16,
        component: u16,
    ) -> OpHeld {
        return OpHeld {
            op,
            obj,
            slot,
            component,
        }
    }
}

impl MessageDecoder<OpHeld> for OpHeld {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpHeld {
        let op: u8 = match prot {
            ClientProt::OPHELD1 => 1,
            ClientProt::OPHELD2 => 2,
            ClientProt::OPHELD3 => 3,
            ClientProt::OPHELD4 => 4,
            ClientProt::OPHELD5 => 5,
            _ => 0,
        };
        return OpHeld::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpHeldT {
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpHeldT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        obj: u16,
        slot: u16,
        component: u16,
        spell: u16,
    ) -> OpHeldT {
        return OpHeldT {
            obj,
            slot,
            component,
            spell,
        }
    }
}

impl MessageDecoder<OpHeldT> for OpHeldT {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpHeldT {
        return OpHeldT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpHeldU {
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpHeldU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        obj: u16,
        slot: u16,
        component: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpHeldU {
        return OpHeldU {
            obj,
            slot,
            component,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpHeldU> for OpHeldU {
    fn length() -> i32 {
        return 12;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpHeldU {
        return OpHeldU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}