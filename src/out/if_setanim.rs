use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetAnim {
    component: i32,
    seq: i32,
}

impl IfSetAnim {
    #[inline]
    pub const fn new(
        component: i32,
        seq: i32,
    ) -> IfSetAnim {
        return IfSetAnim {
            component,
            seq,
        }
    }
}

impl MessageEncoder for IfSetAnim {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETANIM as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.seq);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}