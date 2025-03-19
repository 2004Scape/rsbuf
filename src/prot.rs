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
}

#[repr(u16)]
#[derive(Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum NpcInfoProt {
    ANIM = 0x2,
    FACE_ENTITY = 0x4,
    SAY = 0x8,
    DAMAGE = 0x10,
    CHANGE_TYPE = 0x20,
    SPOT_ANIM = 0x40,
    FACE_COORD = 0x80,
}