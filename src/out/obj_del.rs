use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct ObjDel {
    coord: i32,
    obj: i32,
}

impl ObjDel {
    #[inline]
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
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::OBJ_DEL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 3;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p2(self.obj);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }
}