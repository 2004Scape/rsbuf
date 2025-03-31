use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct LocDel {
    coord: i32,
    shape: i32,
    angle: i32,
}

impl LocDel {
    #[inline]
    pub fn new(
        coord: i32,
        shape: i32,
        angle: i32,
    ) -> LocDel {
        return LocDel {
            coord,
            shape,
            angle,
        }
    }
}

impl MessageEncoder for LocDel {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::LOC_DEL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p1((self.shape << 2) | (self.angle & 0x3));
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}