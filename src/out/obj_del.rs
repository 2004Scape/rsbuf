use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ObjDel {
    coord: i32,
    obj: i32,
}

impl ObjDel {
    pub fn new(
        coord: i32,
        obj: i32,
    ) -> ObjDel {
        return ObjDel {
            coord,
            obj,
        }
    }
}

impl MessageEncoder for ObjDel {
    fn id(&self) -> i32 {
        return ServerInternalProt::OBJ_DEL as i32;
    }

    fn length(&self) -> i32 {
        return 3;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p2(self.obj);
    }

    fn test(&self) -> usize {
        return 3;
    }
}