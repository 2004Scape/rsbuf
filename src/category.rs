#![allow(non_camel_case_types)]

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u8)]
#[wasm_bindgen]
// todo: measure how many events we should expect to receive from the client
// osrs has this as 50/10 but we know that's not true in rs2
// todo: determine which packets belong in which category for this era
pub enum ClientProtCategory {
    CLIENT_EVENT = 0,
    USER_EVENT = 1,
    RESTRICTED_EVENT = 2, // flood restricted events
}

#[repr(u8)]
#[wasm_bindgen]
// packet decoding limit per tick, exceeding this ends decoding and picks up where it left off on the next tick
pub enum ClientProtCategoryLimit {
    CLIENT_EVENT = 20,
    USER_EVENT = 5,
    RESTRICTED_EVENT = 2,
}