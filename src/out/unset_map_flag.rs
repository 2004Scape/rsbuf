use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UnsetMapFlag {}

impl UnsetMapFlag {
    #[inline]
    pub fn new() -> UnsetMapFlag {
        return UnsetMapFlag {}
    }
}

impl MessageEncoder for UnsetMapFlag {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UNSET_MAP_FLAG as i32;
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