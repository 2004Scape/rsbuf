use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ObjAdd {
    coord: i32,
    obj: i32,
    count: i32,
}

impl ObjAdd {
    #[inline]
    pub fn new(
        coord: i32,
        obj: i32,
        count: i32,
    ) -> ObjAdd {
        return ObjAdd {
            coord,
            obj,
            count,
        }
    }
}

impl MessageEncoder for ObjAdd {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::OBJ_ADD as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 5;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p2(self.obj);
        buf.p2(self.count.min(65535));
    }

    #[inline]
    fn test(&self) -> usize {
        return 5;
    }
}