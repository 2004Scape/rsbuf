use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetPlayerHead {
    component: i32,
}

impl IfSetPlayerHead {
    #[inline]
    pub const fn new(
        component: i32,
    ) -> IfSetPlayerHead {
        return IfSetPlayerHead {
            component,
        }
    }
}

impl MessageEncoder for IfSetPlayerHead {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETPLAYERHEAD as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}