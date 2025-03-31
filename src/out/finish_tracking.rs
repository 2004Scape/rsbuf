use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct FinishTracking {}

impl FinishTracking {
    pub fn new() -> FinishTracking {
        return FinishTracking {}
    }
}

impl MessageEncoder for FinishTracking {
    fn id(&self) -> i32 {
        return ServerInternalProt::FINISH_TRACKING as i32;
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