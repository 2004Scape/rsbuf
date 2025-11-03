use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct AnticheatOp1 {}

impl AnticheatOp1 {
    const DEFAULT: AnticheatOp1 = AnticheatOp1::new();

    #[inline]
    pub const fn new() -> AnticheatOp1 {
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
        return AnticheatOp1::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp2 {}

impl AnticheatOp2 {
    const DEFAULT: AnticheatOp2 = AnticheatOp2::new();

    #[inline]
    pub const fn new() -> AnticheatOp2 {
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
        return AnticheatOp2::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp3 {}

impl AnticheatOp3 {
    const DEFAULT: AnticheatOp3 = AnticheatOp3::new();

    #[inline]
    pub const fn new() -> AnticheatOp3 {
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
        return AnticheatOp3::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp4 {}

impl AnticheatOp4 {
    const DEFAULT: AnticheatOp4 = AnticheatOp4::new();

    #[inline]
    pub const fn new() -> AnticheatOp4 {
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
        return AnticheatOp4::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp5 {}

impl AnticheatOp5 {
    const DEFAULT: AnticheatOp5 = AnticheatOp5::new();

    #[inline]
    pub const fn new() -> AnticheatOp5 {
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
        return AnticheatOp5::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp6 {}

impl AnticheatOp6 {
    const DEFAULT: AnticheatOp6 = AnticheatOp6::new();

    #[inline]
    pub const fn new() -> AnticheatOp6 {
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
        return AnticheatOp6::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp7 {}

impl AnticheatOp7 {
    const DEFAULT: AnticheatOp7 = AnticheatOp7::new();

    #[inline]
    pub const fn new() -> AnticheatOp7 {
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
        return AnticheatOp7::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp8 {}

impl AnticheatOp8 {
    const DEFAULT: AnticheatOp8 = AnticheatOp8::new();

    #[inline]
    pub const fn new() -> AnticheatOp8 {
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
        return AnticheatOp8::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp9 {}

impl AnticheatOp9 {
    const DEFAULT: AnticheatOp9 = AnticheatOp9::new();

    #[inline]
    pub const fn new() -> AnticheatOp9 {
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
        return AnticheatOp9::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle1 {}

impl AnticheatCycle1 {
    const DEFAULT: AnticheatCycle1 = AnticheatCycle1::new();

    #[inline]
    pub const fn new() -> AnticheatCycle1 {
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
        return AnticheatCycle1::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle2 {}

impl AnticheatCycle2 {
    const DEFAULT: AnticheatCycle2 = AnticheatCycle2::new();

    #[inline]
    pub const fn new() -> AnticheatCycle2 {
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
        return AnticheatCycle2::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle3 {}

impl AnticheatCycle3 {
    const DEFAULT: AnticheatCycle3 = AnticheatCycle3::new();

    #[inline]
    pub const fn new() -> AnticheatCycle3 {
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
        return AnticheatCycle3::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle4 {}

impl AnticheatCycle4 {
    const DEFAULT: AnticheatCycle4 = AnticheatCycle4::new();

    #[inline]
    pub const fn new() -> AnticheatCycle4 {
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
        return AnticheatCycle4::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle5 {}

impl AnticheatCycle5 {
    const DEFAULT: AnticheatCycle5 = AnticheatCycle5::new();

    #[inline]
    pub const fn new() -> AnticheatCycle5 {
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
        return AnticheatCycle5::DEFAULT;
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle6 {}

impl AnticheatCycle6 {
    const DEFAULT: AnticheatCycle6 = AnticheatCycle6::new();

    #[inline]
    pub const fn new() -> AnticheatCycle6 {
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
        return AnticheatCycle6::DEFAULT;
    }
}