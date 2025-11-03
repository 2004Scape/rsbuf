use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct FinishTracking {}

impl FinishTracking {
    #[inline]
    pub const fn new() -> FinishTracking {
        return FinishTracking {}
    }
}

impl MessageEncoder for FinishTracking {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::FINISH_TRACKING as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}