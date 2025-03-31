use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct EnableTracking {}

impl EnableTracking {
    #[inline]
    pub fn new() -> EnableTracking {
        return EnableTracking {}
    }
}

impl MessageEncoder for EnableTracking {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::ENABLE_TRACKING as i32;
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