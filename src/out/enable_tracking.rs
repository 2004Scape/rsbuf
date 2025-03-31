use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct EnableTracking {}

impl EnableTracking {
    pub fn new() -> EnableTracking {
        return EnableTracking {}
    }
}

impl MessageEncoder for EnableTracking {
    fn id(&self) -> i32 {
        return ServerInternalProt::ENABLE_TRACKING as i32;
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