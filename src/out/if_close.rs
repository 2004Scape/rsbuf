use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfClose {}

impl IfClose {
    pub fn new() -> IfClose {
        return IfClose {}
    }
}

impl MessageEncoder for IfClose {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_CLOSE as i32;
    }

    fn length(&self) -> i32 {
        return 0;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, _: &mut Packet) {}

    fn test(&self) -> usize {
        return 0;
    }
}