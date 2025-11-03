use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct FriendListAdd {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

impl FriendListAdd {
    #[inline]
    pub fn new(username: i64) -> FriendListAdd {
        return FriendListAdd {
            username,
        }
    }
}

impl MessageDecoder<FriendListAdd> for FriendListAdd {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> FriendListAdd {
        return FriendListAdd::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct FriendListDel {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

impl FriendListDel {
    #[inline]
    pub fn new(username: i64) -> FriendListDel {
        return FriendListDel {
            username,
        }
    }
}

impl MessageDecoder<FriendListDel> for FriendListDel {
    #[inline]
    fn length() -> i32 {
        return 8;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> FriendListDel {
        return FriendListDel::new(buf.g8s());
    }
}