use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ClientCheat {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: String,
}

#[wasm_bindgen]
impl ClientCheat {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> ClientCheat {
        return ClientCheat {
            input,
        }
    }
}

impl MessageDecoder<ClientCheat> for ClientCheat {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ClientCheat {
        return ClientCheat::new(buf.gjstr(10));
    }
}