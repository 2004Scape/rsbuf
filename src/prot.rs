#![allow(non_camel_case_types)]

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u16)]
#[derive(Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum PlayerInfoProt {
    BIG = 0x10,
    ANIM = 0x8,
    SAY = 0x20,
    EXACT_MOVE = 0x400,
    FACE_ENTITY = 0x2,
    FACE_COORD = 0x40,
    SPOT_ANIM = 0x100,
    APPEARANCE = 0x4,
    DAMAGE = 0x1,
    CHAT = 0x80,
    DAMAGE2 = 0x200,
}

impl PlayerInfoProt {
    #[inline]
    pub const fn to_index(self) -> usize {
        // the ordering here does not matter.
        return match self {
            PlayerInfoProt::APPEARANCE => 0,
            PlayerInfoProt::ANIM => 1,
            PlayerInfoProt::FACE_ENTITY => 2,
            PlayerInfoProt::SAY => 3,
            PlayerInfoProt::DAMAGE => 4,
            PlayerInfoProt::DAMAGE2 => 5,
            PlayerInfoProt::FACE_COORD => 6,
            PlayerInfoProt::CHAT => 7,
            PlayerInfoProt::SPOT_ANIM => 8,
            PlayerInfoProt::BIG => 255, // unused
            PlayerInfoProt::EXACT_MOVE => 255, // unused
        }
    }
}

#[repr(u16)]
#[derive(Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum NpcInfoProt {
    CHANGE_TYPE = 1,
    FACE_ENTITY = 64,
    DAMAGE = 128,
    SPOT_ANIM = 4,
    SAY = 32,
    FACE_COORD = 8,
    ANIM = 2,
    DAMAGE2 = 16,
}

impl NpcInfoProt {
    #[inline]
    pub const fn to_index(self) -> usize {
        // the ordering here does not matter.
        return match self {
            NpcInfoProt::ANIM => 0,
            NpcInfoProt::FACE_ENTITY => 1,
            NpcInfoProt::SAY => 2,
            NpcInfoProt::DAMAGE => 3,
            NpcInfoProt::DAMAGE2 => 4,
            NpcInfoProt::CHANGE_TYPE => 5,
            NpcInfoProt::SPOT_ANIM => 6,
            NpcInfoProt::FACE_COORD => 7,
        }
    }
}