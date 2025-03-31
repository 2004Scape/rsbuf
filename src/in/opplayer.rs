use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct OpPlayer {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub pid: u16,
}

#[wasm_bindgen]
impl OpPlayer {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        op: u8,
        pid: u16,
    ) -> OpPlayer {
        return OpPlayer {
            op,
            pid,
        }
    }
}

impl MessageDecoder<OpPlayer> for OpPlayer {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(prot: ClientProt, buf: &mut Packet) -> OpPlayer {
        let op: u8 = match prot {
            ClientProt::OPPLAYER1 => 1,
            ClientProt::OPPLAYER2 => 2,
            ClientProt::OPPLAYER3 => 3,
            ClientProt::OPPLAYER4 => 4,
            _ => 0,
        };
        return OpPlayer::new(
            op,
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpPlayerT {
    #[wasm_bindgen(readonly)]
    pub pid: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpPlayerT {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        pid: u16,
        spell: u16,
    ) -> OpPlayerT {
        return OpPlayerT {
            pid,
            spell,
        }
    }
}

impl MessageDecoder<OpPlayerT> for OpPlayerT {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, buf: &mut Packet) -> OpPlayerT {
        return OpPlayerT::new(
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpPlayerU {
    #[wasm_bindgen(readonly)]
    pub pid: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpPlayerU {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        pid: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpPlayerU {
        return OpPlayerU {
            pid,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpPlayerU> for OpPlayerU {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, buf: &mut Packet) -> OpPlayerU {
        return OpPlayerU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}