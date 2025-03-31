use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UnsetMapFlag {}

impl UnsetMapFlag {
    pub fn new() -> UnsetMapFlag {
        return UnsetMapFlag {}
    }
}

impl MessageEncoder for UnsetMapFlag {
    fn id(&self) -> i32 {
        return ServerInternalProt::UNSET_MAP_FLAG as i32;
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