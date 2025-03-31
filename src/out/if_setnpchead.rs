use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetNpcHead {
    component: i32,
    npc: i32,
}

impl IfSetNpcHead {
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
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETNPCHEAD as i32;
    }

    fn length(&self) -> i32 {
        return 4;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.npc);
    }

    fn test(&self) -> usize {
        return 4;
    }
}