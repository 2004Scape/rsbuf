use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfOpenSide {
    component: i32,
}

impl IfOpenSide {
    pub fn new(component:i32) -> IfOpenSide {
        return IfOpenSide {
            component,
        }
    }
}

impl MessageEncoder for IfOpenSide {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENSIDE as i32
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    fn test(&self) -> usize {
        return 2;
    }
}