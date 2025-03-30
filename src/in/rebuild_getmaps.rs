use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct RebuildGetMaps {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub maps: Vec<u32>,
}

#[wasm_bindgen]
impl RebuildGetMaps {
    #[wasm_bindgen(constructor)]
    pub fn new(maps: Vec<u32>) -> RebuildGetMaps {
        return RebuildGetMaps {
            maps,
        }
    }
}

impl MessageDecoder<RebuildGetMaps> for RebuildGetMaps {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> RebuildGetMaps {
        let mut maps: Vec<u32> = vec![0; buf.data.len() / 3];
        for index in 0..maps.len() {
            unsafe { *maps.as_mut_ptr().add(index) = buf.g3() as u32 };
        }
        return RebuildGetMaps::new(maps);
    }
}