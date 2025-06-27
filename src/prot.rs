#![allow(non_camel_case_types)]

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u16)]
#[derive(Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum PlayerInfoProt {
    APPEARANCE = 0x1,
    ANIM = 0x2,
    FACE_ENTITY = 0x4,
    SAY = 0x8,
    DAMAGE = 0x10,
    FACE_COORD = 0x20,
    CHAT = 0x40,
    BIG = 0x80,
    SPOT_ANIM = 0x100,
    EXACT_MOVE = 0x200,
    DAMAGE2 = 0x400,
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
    DAMAGE2 = 0x1,
    ANIM = 0x2,
    FACE_ENTITY = 0x4,
    SAY = 0x8,
    DAMAGE = 0x10,
    CHANGE_TYPE = 0x20,
    SPOT_ANIM = 0x40,
    FACE_COORD = 0x80,
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