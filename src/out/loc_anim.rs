use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct LocAnim {
    coord: i32,
    shape: i32,
    angle: i32,
    seq: i32,
}

impl LocAnim {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::LOC_ANIM as i32;
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
        buf.p2(self.seq);
    }

    fn test(&self) -> usize {
        return 4;
    }
}