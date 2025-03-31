use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetNpcHead {
    component: i32,
    npc: i32,
}

impl IfSetNpcHead {
    #[inline]
    pub fn new(
        component: i32,
        npc: i32,
    ) -> IfSetNpcHead {
        return IfSetNpcHead {
            component,
            npc,
        }
    }
}

impl MessageEncoder for IfSetNpcHead {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETNPCHEAD as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.npc);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}