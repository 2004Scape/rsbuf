use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ResetClientVarCache {}

impl ResetClientVarCache {
    pub fn new() -> ResetClientVarCache {
        return ResetClientVarCache {}
    }
}

impl MessageEncoder for ResetClientVarCache {
    fn id(&self) -> i32 {
        return ServerInternalProt::RESET_CLIENT_VARCACHE as i32;
    }

    fn length(&self) -> i32 {
        return 0;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, _: &mut Packet) {}

    fn test(&self) -> usize {
        return 0;
    }
}