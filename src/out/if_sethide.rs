use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetHide {
    component: i32,
    hidden: bool,
}

impl IfSetHide {
    pub fn new(
        component: i32,
        hidden: bool,
    ) -> IfSetHide {
        return IfSetHide {
            component,
            hidden,
        }
    }
}

impl MessageEncoder for IfSetHide {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETHIDE as i32;
    }

    fn length(&self) -> i32 {
        return 3;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p1(if self.hidden { 1 } else { 0 });
    }

    fn test(&self) -> usize {
        return 3;
    }
}