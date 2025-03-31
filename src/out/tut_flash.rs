use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct TutFlash {
    tab: i32,
}

impl TutFlash {
    #[inline]
    pub fn new(
        tab: i32,
    ) -> TutFlash {
        return TutFlash {
            tab,
        }
    }
}

impl MessageEncoder for TutFlash {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::TUT_FLASH as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 1;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.tab);
    }

    #[inline]
    fn test(&self) -> usize {
        return 1;
    }
}