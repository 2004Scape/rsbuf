use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ClientCheat {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: String,
}

impl ClientCheat {
    #[inline]
    pub fn new(input: String) -> ClientCheat {
        return ClientCheat {
            input,
        }
    }
}

impl MessageDecoder<ClientCheat> for ClientCheat {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> ClientCheat {
        return ClientCheat::new(buf.gjstr(10));
    }
}