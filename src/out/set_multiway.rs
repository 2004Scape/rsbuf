use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct SetMultiway {
    hidden: bool,
}

impl SetMultiway {
    #[inline]
    pub const fn new(hidden: bool) -> SetMultiway {
        return SetMultiway {
            hidden,
        }
    }
}

impl MessageEncoder for SetMultiway {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::SET_MULTIWAY as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 1;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(if self.hidden { 1 } else { 0 });
    }

    #[inline]
    fn test(&self) -> usize {
        return 1;
    }
}