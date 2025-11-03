use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdateInvFull {
    size: i32,
    component: i32,
    inv: Vec<i64>,
}

impl UpdateInvFull {
    #[inline]
    pub const fn new(
        size: i32,
        component: i32,
        inv: Vec<i64>,
    ) -> UpdateInvFull {
        return UpdateInvFull {
            size,
            component,
            inv,
        }
    }
}

impl MessageEncoder for UpdateInvFull {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_INV_FULL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        // todo: size should be the index of the last non-empty slot
        buf.p2(self.component);
        buf.p1(self.size);
        for &packed in &self.inv[..self.size as usize] {
            let obj: i32 = (packed >> 31) as i32;
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
        let mut length: usize = 3;
        for &packed in &self.inv[..self.size as usize] {
            let obj: i32 = (packed >> 31) as i32;
            if obj != -1 {
                let count = (packed & 0x7fffffff) as i32;
                length += 2 + if count >= 0xff { 5 } else { 1 };
            } else {
                length += 3;
            }
        }
        return length
    }
}