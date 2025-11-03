use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdateInvPartial {
    component: i32,
    slots: Vec<i32>,
    inv: Vec<i64>,
}

impl UpdateInvPartial {
    #[inline]
    pub const fn new(
        component: i32,
        slots: Vec<i32>,
        inv: Vec<i64>,
    ) -> UpdateInvPartial {
        return UpdateInvPartial {
            component,
            slots,
            inv,
        }
    }
}

impl MessageEncoder for UpdateInvPartial {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_INV_PARTIAL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        // todo: size should be the index of the last non-empty slot
        buf.p2(self.component);
        for &slot in &self.slots {
            let packed: i64 = self.inv[slot as usize];
            let obj: i32 = (packed >> 31) as i32;
            buf.p1(slot);
            if obj != -1 {
                let count: i32 = (packed & 0x7fffffff) as i32;
                buf.p2(obj + 1);
                if count >= 0xff {
                    buf.p1(0xff);
                    buf.p4(count);
                } else {
                    buf.p1(count);
                }
            } else {
                buf.p2(0);
                buf.p1(0);
            }
        }
    }

    #[inline]
    fn test(&self) -> usize {
        let mut length: usize = 2;
        for &slot in &self.slots {
            let packed: i64 = self.inv[slot as usize];
            let obj: i32 = (packed >> 31) as i32;
            length += 1;
            if obj != -1 {
                let count: i32 = (packed & 0x7fffffff) as i32;
                length += if count >= 0xff { 5 } else { 1 };
            } else {
                length += 3;
            }
        }
        return length;
    }
}