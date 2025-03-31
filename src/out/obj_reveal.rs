use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ObjReveal {
    coord: i32,
    obj: i32,
    count: i32,
    receiver: i32,
}

impl ObjReveal {
    pub fn new(
        coord: i32,
        obj: i32,
        count: i32,
        receiver: i32,
    ) -> ObjReveal {
        return ObjReveal {
            coord,
            obj,
            count,
            receiver,
        }
    }
}

impl MessageEncoder for ObjReveal {
    fn id(&self) -> i32 {
        return ServerInternalProt::OBJ_REVEAL as i32;
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
        buf.p2(self.count.min(65535));
        buf.p2(self.receiver);
    }

    fn test(&self) -> usize {
        return 7;
    }
}