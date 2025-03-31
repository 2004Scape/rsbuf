use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct TutOpen {
    component: i32,
}

impl TutOpen {
    pub fn new(
        component: i32,
    ) -> TutOpen {
        return TutOpen {
            component,
        }
    }
}

impl MessageEncoder for TutOpen {
    fn id(&self) -> i32 {
        return ServerInternalProt::TUT_OPEN as i32;
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