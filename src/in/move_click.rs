use crate::coord::CoordGrid;
use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MoveClick {
    #[wasm_bindgen(readonly)]
    pub ctrl: bool,
    #[wasm_bindgen(readonly)]
    pub op: bool,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub path: Vec<u32>,
}

impl MoveClick {
    #[inline]
    pub fn new(
        ctrl: bool,
        op: bool,
        path: Vec<u32>,
    ) -> MoveClick {
        return MoveClick {
            ctrl,
            op,
            path,
        }
    }
}

impl MessageDecoder<MoveClick> for MoveClick {
    #[inline]
    fn length() -> i32 {
        return -1;
    }

    #[inline]
    fn decode(prot: ClientProt, mut buf: Packet) -> MoveClick {
        let ctrl: bool = buf.g1() == 1;
        let x: u16 = buf.g2();
        let z: u16 = buf.g2();

        let offset: usize = if prot == ClientProt::MOVE_MINIMAPCLICK { 14 } else { 0 };
        let waypoints: usize = ((buf.data.len() - buf.pos - offset) / 2) + 1;

        let mut path: Vec<u32> = vec![0; waypoints];
        path[0] = CoordGrid::from(x, 0, z).packed;

        for index in 1..waypoints {
            if index >= 25 {
                break;
            }
            unsafe { *path.as_mut_ptr().add(index) = CoordGrid::from(x + buf.g1s() as u16, 0, z + buf.g1s() as u16).packed };
        }

        return MoveClick::new(
            ctrl,
            prot == ClientProt::MOVE_OPCLICK,
            path,
        );
    }
}