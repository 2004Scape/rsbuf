use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ResumePCountDialog {
    #[wasm_bindgen(readonly)]
    pub input: i32,
}

#[wasm_bindgen]
impl ResumePCountDialog {
    #[wasm_bindgen(constructor)]
    pub fn new(input: i32) -> ResumePCountDialog {
        return ResumePCountDialog {
            input,
        }
    }
}

impl MessageDecoder<ResumePCountDialog> for ResumePCountDialog {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ResumePCountDialog {
        return ResumePCountDialog::new(buf.g4s());
    }
}