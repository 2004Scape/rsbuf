use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct EventTracking {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub bytes: Vec<u8>,
}

#[wasm_bindgen]
impl EventTracking {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(bytes: Vec<u8>) -> EventTracking {
        return EventTracking {
            bytes,
        }
    }
}

impl MessageDecoder<EventTracking> for EventTracking {
    #[inline]
    fn length() -> i32 {
        return -2;
    }

    #[inline]
    fn decode(_: ClientProt, buf: Packet) -> EventTracking {
        return EventTracking::new(buf.data.clone());
    }
}

// ----

#[wasm_bindgen]
pub struct EventCameraPosition {
    #[wasm_bindgen(readonly)]
    pub pitch: i32,
    #[wasm_bindgen(readonly)]
    pub yaw: i32,
    #[wasm_bindgen(readonly)]
    pub angle: i32,
    #[wasm_bindgen(readonly)]
    pub zoom: i32,
}

#[wasm_bindgen]
impl EventCameraPosition {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new(
        pitch: i32,
        yaw: i32,
        angle: i32,
        zoom: i32,
    ) -> EventCameraPosition {
        return EventCameraPosition {
            pitch,
            yaw,
            angle,
            zoom,
        }
    }
}

impl MessageDecoder<EventCameraPosition> for EventCameraPosition {
    #[inline]
    fn length() -> i32 {
        return 6;
    }

    #[inline]
    fn decode(_: ClientProt, mut buf: Packet) -> EventCameraPosition {
        return EventCameraPosition::new(
            buf.g2() as i32,
            buf.g2() as i32,
            buf.g1() as i32,
            buf.g1() as i32,
        );
    }
}