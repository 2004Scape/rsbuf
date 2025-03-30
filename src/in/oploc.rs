use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct OpLoc {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
}

#[wasm_bindgen]
impl OpLoc {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        x: u16,
        z: u16,
        loc: u16,
    ) -> OpLoc {
        return OpLoc {
            op,
            x,
            z,
            loc,
        }
    }
}

impl MessageDecoder<OpLoc> for OpLoc {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpLoc {
        let op: u8 = match prot {
            ClientProt::OPLOC1 => 1,
            ClientProt::OPLOC2 => 2,
            ClientProt::OPLOC3 => 3,
            ClientProt::OPLOC4 => 4,
            ClientProt::OPLOC5 => 5,
            _ => 0,
        };
        return OpLoc::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpLocT {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpLocT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        loc: u16,
        spell: u16,
    ) -> OpLocT {
        return OpLocT {
            x,
            z,
            loc,
            spell,
        }
    }
}

impl MessageDecoder<OpLocT> for OpLocT {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpLocT {
        return OpLocT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpLocU {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpLocU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        loc: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpLocU {
        return OpLocU {
            x,
            z,
            loc,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpLocU> for OpLocU {
    fn length() -> i32 {
        return 12;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpLocU {
        return OpLocU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}