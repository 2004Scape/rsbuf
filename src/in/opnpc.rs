use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct OpNpc {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub nid: u16,
}

#[wasm_bindgen]
impl OpNpc {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        op: u8,
        nid: u16,
    ) -> OpNpc {
        return OpNpc {
            op,
            nid,
        }
    }
}

impl MessageDecoder<OpNpc> for OpNpc {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(prot: ClientProt, mut buf: Packet) -> OpNpc {
        let op: u8 = match prot {
            ClientProt::OPNPC1 => 1,
            ClientProt::OPNPC2 => 2,
            ClientProt::OPNPC3 => 3,
            ClientProt::OPNPC4 => 4,
            ClientProt::OPNPC5 => 5,
            _ => 0,
        };
        return OpNpc::new(
            op,
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpNpcT {
    #[wasm_bindgen(readonly)]
    pub nid: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpNpcT {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        nid: u16,
        spell: u16,
    ) -> OpNpcT {
        return OpNpcT {
            nid,
            spell,
        }
    }
}

impl MessageDecoder<OpNpcT> for OpNpcT {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> OpNpcT {
        return OpNpcT::new(
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpNpcU {
    #[wasm_bindgen(readonly)]
    pub nid: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpNpcU {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        nid: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpNpcU {
        return OpNpcU {
            nid,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpNpcU> for OpNpcU {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> OpNpcU {
        return OpNpcU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}