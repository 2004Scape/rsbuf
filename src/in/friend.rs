use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct FriendListAdd {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl FriendListAdd {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> FriendListAdd {
        return FriendListAdd {
            username,
        }
    }
}

impl MessageDecoder<FriendListAdd> for FriendListAdd {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> FriendListAdd {
        return FriendListAdd::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct FriendListDel {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl FriendListDel {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> FriendListDel {
        return FriendListDel {
            username,
        }
    }
}

impl MessageDecoder<FriendListDel> for FriendListDel {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> FriendListDel {
        return FriendListDel::new(buf.g8s());
    }
}