use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfClose {}

impl IfClose {
    #[inline]
    pub fn new() -> IfClose {
        return IfClose {}
    }
}

impl MessageEncoder for IfClose {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_CLOSE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}