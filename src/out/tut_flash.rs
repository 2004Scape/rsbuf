use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct TutFlash {
    tab: i32,
}

impl TutFlash {
    pub fn new(
        tab: i32,
    ) -> TutFlash {
        return TutFlash {
            tab,
        }
    }
}

impl MessageEncoder for TutFlash {
    fn id(&self) -> i32 {
        return ServerInternalProt::TUT_FLASH as i32;
    }

    fn length(&self) -> i32 {
        return 1;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.tab);
    }

    fn test(&self) -> usize {
        return 1;
    }
}