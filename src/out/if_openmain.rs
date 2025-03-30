use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfOpenMain {
    component: i32,
}

impl IfOpenMain {
    pub fn new(component:i32) -> IfOpenMain {
        return IfOpenMain {
            component,
        }
    }
}

impl MessageEncoder for IfOpenMain {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENMAIN as i32;
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