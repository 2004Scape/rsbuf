use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetAnim {
    component: i32,
    seq: i32,
}

impl IfSetAnim {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETANIM as i32;
    }

    fn length(&self) -> i32 {
        return 4;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.seq);
    }

    fn test(&self) -> usize {
        return 4;
    }
}