use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct AnticheatOp1 {}

#[wasm_bindgen]
impl AnticheatOp1 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp1 {
        return AnticheatOp1 {};
    }
}

impl MessageDecoder<AnticheatOp1> for AnticheatOp1 {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp1 {
        return AnticheatOp1::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp2 {}

#[wasm_bindgen]
impl AnticheatOp2 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp2 {
        return AnticheatOp2 {};
    }
}

impl MessageDecoder<AnticheatOp2> for AnticheatOp2 {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp2 {
        return AnticheatOp2::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp3 {}

#[wasm_bindgen]
impl AnticheatOp3 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp3 {
        return AnticheatOp3 {};
    }
}

impl MessageDecoder<AnticheatOp3> for AnticheatOp3 {
    #[inline]
    fn length() -> i32 {
        return 3;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp3 {
        return AnticheatOp3::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp4 {}

#[wasm_bindgen]
impl AnticheatOp4 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp4 {
        return AnticheatOp4 {};
    }
}

impl MessageDecoder<AnticheatOp4> for AnticheatOp4 {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp4 {
        return AnticheatOp4::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp5 {}

#[wasm_bindgen]
impl AnticheatOp5 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp5 {
        return AnticheatOp5 {};
    }
}

impl MessageDecoder<AnticheatOp5> for AnticheatOp5 {
    #[inline]
    fn length() -> i32 {
        return 0;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp5 {
        return AnticheatOp5::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp6 {}

#[wasm_bindgen]
impl AnticheatOp6 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp6 {
        return AnticheatOp6 {};
    }
}

impl MessageDecoder<AnticheatOp6> for AnticheatOp6 {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp6 {
        return AnticheatOp6::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp7 {}

#[wasm_bindgen]
impl AnticheatOp7 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp7 {
        return AnticheatOp7 {};
    }
}

impl MessageDecoder<AnticheatOp7> for AnticheatOp7 {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp7 {
        return AnticheatOp7::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp8 {}

#[wasm_bindgen]
impl AnticheatOp8 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp8 {
        return AnticheatOp8 {};
    }
}

impl MessageDecoder<AnticheatOp8> for AnticheatOp8 {
    #[inline]
    fn length() -> i32 {
        return 2;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp8 {
        return AnticheatOp8::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp9 {}

#[wasm_bindgen]
impl AnticheatOp9 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatOp9 {
        return AnticheatOp9 {};
    }
}

impl MessageDecoder<AnticheatOp9> for AnticheatOp9 {
    #[inline]
    fn length() -> i32 {
        return 1;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatOp9 {
        return AnticheatOp9::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle1 {}

#[wasm_bindgen]
impl AnticheatCycle1 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle1 {
        return AnticheatCycle1 {};
    }
}

impl MessageDecoder<AnticheatCycle1> for AnticheatCycle1 {
    #[inline]
    fn length() -> i32 {
        return 1;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle1 {
        return AnticheatCycle1::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle2 {}

#[wasm_bindgen]
impl AnticheatCycle2 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle2 {
        return AnticheatCycle2 {};
    }
}

impl MessageDecoder<AnticheatCycle2> for AnticheatCycle2 {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle2 {
        return AnticheatCycle2::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle3 {}

#[wasm_bindgen]
impl AnticheatCycle3 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle3 {
        return AnticheatCycle3 {};
    }
}

impl MessageDecoder<AnticheatCycle3> for AnticheatCycle3 {
    #[inline]
    fn length() -> i32 {
        return 3;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle3 {
        return AnticheatCycle3::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle4 {}

#[wasm_bindgen]
impl AnticheatCycle4 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle4 {
        return AnticheatCycle4 {};
    }
}

impl MessageDecoder<AnticheatCycle4> for AnticheatCycle4 {
    #[inline]
    fn length() -> i32 {
        return 4;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle4 {
        return AnticheatCycle4::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle5 {}

#[wasm_bindgen]
impl AnticheatCycle5 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle5 {
        return AnticheatCycle5 {};
    }
}

impl MessageDecoder<AnticheatCycle5> for AnticheatCycle5 {
    #[inline]
    fn length() -> i32 {
        return 0;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle5 {
        return AnticheatCycle5::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle6 {}

#[wasm_bindgen]
impl AnticheatCycle6 {
    #[wasm_bindgen(constructor)]
    #[inline]
    pub fn new() -> AnticheatCycle6 {
        return AnticheatCycle6 {};
    }
}

impl MessageDecoder<AnticheatCycle6> for AnticheatCycle6 {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(_: ClientProt, _: Packet) -> AnticheatCycle6 {
        return AnticheatCycle6::new();
    }
}