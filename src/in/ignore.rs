use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IgnoreListAdd {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl IgnoreListAdd {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(username: i64) -> IgnoreListAdd {
        return IgnoreListAdd {
            username,
        }
    }
}

impl MessageDecoder<IgnoreListAdd> for IgnoreListAdd {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, buf: &mut Packet) -> IgnoreListAdd {
        return IgnoreListAdd::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct IgnoreListDel {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl IgnoreListDel {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(username: i64) -> IgnoreListDel {
        return IgnoreListDel {
            username,
        }
    }
}

impl MessageDecoder<IgnoreListDel> for IgnoreListDel {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, buf: &mut Packet) -> IgnoreListDel {
        return IgnoreListDel::new(buf.g8s());
    }
}