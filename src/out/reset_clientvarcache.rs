use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ResetClientVarCache {}

impl ResetClientVarCache {
    #[inline]
    pub fn new() -> ResetClientVarCache {
        return ResetClientVarCache {}
    }
}

impl MessageEncoder for ResetClientVarCache {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::RESET_CLIENT_VARCACHE as i32;
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