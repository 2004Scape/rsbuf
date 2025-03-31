use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct Logout {}

impl Logout {
    pub fn new() -> Logout {
        return Logout {}
    }
}

impl MessageEncoder for Logout {
    fn id(&self) -> i32 {
        return ServerInternalProt::LOGOUT as i32;
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