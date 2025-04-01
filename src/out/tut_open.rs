use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct TutOpen {
    component: i32,
}

impl TutOpen {
    #[inline]
    pub const fn new(
        component: i32,
    ) -> TutOpen {
        return TutOpen {
            component,
        }
    }
}

impl MessageEncoder for TutOpen {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::TUT_OPEN as i32;
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