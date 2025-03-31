use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ReportAbuse {
    #[wasm_bindgen(readonly)]
    pub offender: i64,
    #[wasm_bindgen(readonly)]
    pub reason: u8,
    #[wasm_bindgen(readonly)]
    pub mute: bool,
}

#[wasm_bindgen]
impl ReportAbuse {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        offender: i64,
        reason: u8,
        mute: bool,
    ) -> ReportAbuse {
        return ReportAbuse {
            offender,
            reason,
            mute,
        }
    }
}

impl MessageDecoder<ReportAbuse> for ReportAbuse {
    #[inline]
    fn length() -> i32 {
        return 10;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> ReportAbuse {
        return ReportAbuse::new(
            buf.g8s(),
            buf.g1(),
            buf.g1() == 1,
        );
    }
}