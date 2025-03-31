use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct SetMultiway {
    hidden: bool,
}

impl SetMultiway {
    pub fn new(hidden: bool) -> SetMultiway {
        return SetMultiway {
            hidden,
        }
    }
}

impl MessageEncoder for SetMultiway {
    fn id(&self) -> i32 {
        return ServerInternalProt::SET_MULTIWAY as i32;
    }

    fn length(&self) -> i32 {
        return 1;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(if self.hidden { 1 } else { 0 });
    }

    fn test(&self) -> usize {
        return 1;
    }
}