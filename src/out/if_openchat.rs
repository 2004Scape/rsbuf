use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfOpenChat {
    component: i32,
}

impl IfOpenChat {
    #[inline]
    pub fn new(component:i32) -> IfOpenChat {
        return IfOpenChat {
            component,
        }
    }
}

impl MessageEncoder for IfOpenChat {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENCHAT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}