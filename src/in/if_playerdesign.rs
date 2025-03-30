use crate::message::MessageDecoder;
use crate::packet::Packet;
use crate::prot::ClientProt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct IfPlayerDesign {
    #[wasm_bindgen(readonly)]
    pub gender: u8,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub idkit: Vec<i32>,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub color: Vec<i32>,
}

#[wasm_bindgen]
impl IfPlayerDesign {
    #[wasm_bindgen(constructor)]
    pub fn new(gender: u8, idkit: Vec<i32>, color: Vec<i32>) -> IfPlayerDesign {
        return IfPlayerDesign {
            gender,
            idkit,
            color,
        }
    }
}

impl MessageDecoder<IfPlayerDesign> for IfPlayerDesign {
    fn length() -> i32 {
        return 13;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> IfPlayerDesign {
        let gender: u8 = buf.g1();

        let mut idkit: [i32; 7] = [0; 7];
        for i in 0..7 {
            let mut v = buf.g1() as i32;
            if v == 0xff {
                v = -1;
            }
            unsafe { *idkit.as_mut_ptr().add(i as usize) = v };
        }

        let mut color: [i32; 5] = [0; 5];
        for i in 0..5 {
            unsafe { *color.as_mut_ptr().add(i as usize) = buf.g1() as i32 };
        }

        return IfPlayerDesign::new(gender, idkit.to_vec(), color.to_vec());
    }
}