use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ResetAnims {}

impl ResetAnims {
    #[inline]
    pub fn new() -> ResetAnims {
        return ResetAnims {}
    }
}

impl MessageEncoder for ResetAnims {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::RESET_ANIMS as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate; // todo: what should priority be?
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}