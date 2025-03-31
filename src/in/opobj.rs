use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct OpObj {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
}

#[wasm_bindgen]
impl OpObj {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        op: u8,
        x: u16,
        z: u16,
        obj: u16,
    ) -> OpObj {
        return OpObj {
            op,
            x,
            z,
            obj,
        }
    }
}

impl MessageDecoder<OpObj> for OpObj {
    #[inline]
    fn length() -> i32 {
        return 6;
    }

    #[inline]
    fn decode(prot: ClientProt, mut buf: Packet) -> OpObj {
        let op: u8 = match prot {
            ClientProt::OPOBJ1 => 1,
            ClientProt::OPOBJ2 => 2,
            ClientProt::OPOBJ3 => 3,
            ClientProt::OPOBJ4 => 4,
            ClientProt::OPOBJ5 => 5,
            _ => 0,
        };
        return OpObj::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpObjT {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpObjT {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        x: u16,
        z: u16,
        obj: u16,
        spell: u16,
    ) -> OpObjT {
        return OpObjT {
            x,
            z,
            obj,
            spell,
        }
    }
}

impl MessageDecoder<OpObjT> for OpObjT {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> OpObjT {
        return OpObjT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpObjU {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpObjU {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        x: u16,
        z: u16,
        obj: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpObjU {
        return OpObjU {
            x,
            z,
            obj,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpObjU> for OpObjU {
    #[inline]
    fn length() -> i32 {
        return 12;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> OpObjU {
        return OpObjU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}