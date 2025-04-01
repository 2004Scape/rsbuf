use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct LocAnim {
    coord: i32,
    shape: i32,
    angle: i32,
    seq: i32,
}

impl LocAnim {
    #[inline]
    pub const fn new(
        coord: i32,
        shape: i32,
        angle: i32,
        seq: i32,
    ) -> LocAnim {
        return LocAnim {
            coord,
            shape,
            angle,
            seq,
        }
    }
}

impl MessageEncoder for LocAnim {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::LOC_ANIM as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p1((self.shape << 2) | (self.angle & 0x3));
        buf.p2(self.seq);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}