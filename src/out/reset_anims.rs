use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ResetAnims {}

impl ResetAnims {
    pub fn new() -> ResetAnims {
        return ResetAnims {}
    }
}

impl MessageEncoder for ResetAnims {
    fn id(&self) -> i32 {
        return ServerInternalProt::RESET_ANIMS as i32;
    }

    fn length(&self) -> i32 {
        return 0;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate; // todo: what should priority be?
    }

    fn encode(&self, _: &mut Packet) {}

    fn test(&self) -> usize {
        return 0;
    }
}