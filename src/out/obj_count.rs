use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ObjCount {
    coord: i32,
    obj: i32,
    old_count: i32,
    new_count: i32,
}

impl ObjCount {
    pub fn new(
        coord: i32,
        obj: i32,
        old_count: i32,
        new_count: i32,
    ) -> ObjCount {
        return ObjCount {
            coord,
            obj,
            old_count,
            new_count,
        }
    }
}

impl MessageEncoder for ObjCount {
    fn id(&self) -> i32 {
        return ServerInternalProt::OBJ_COUNT as i32;
    }

    fn length(&self) -> i32 {
        return 7;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p2(self.obj);
        buf.p2(self.old_count.min(65535));
        buf.p2(self.new_count.min(65535));
    }

    fn test(&self) -> usize {
        return 7;
    }
}