use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct LocAddChange {
    coord: i32,
    loc: i32,
    shape: i32,
    angle: i32,
}

impl LocAddChange {
    pub fn new(
        coord: i32,
        loc: i32,
        shape: i32,
        angle: i32,
    ) -> LocAddChange {
        return LocAddChange {
            coord,
            loc,
            shape,
            angle,
        }
    }
}

impl MessageEncoder for LocAddChange {
    fn id(&self) -> i32 {
        return ServerInternalProt::LOC_ADD_CHANGE as i32;
    }

    fn length(&self) -> i32 {
        return 4;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p1((self.shape << 2) | (self.angle & 0x3));
        buf.p2(self.loc);
    }

    fn test(&self) -> usize {
        return 4;
    }
}