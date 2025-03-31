use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct Logout {}

impl Logout {
    #[inline]
    pub fn new() -> Logout {
        return Logout {}
    }
}

impl MessageEncoder for Logout {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::LOGOUT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}