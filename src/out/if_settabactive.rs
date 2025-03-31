use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetTabActive {
    tab: i32,
}

impl IfSetTabActive {
    pub fn new(
        tab: i32,
    ) -> IfSetTabActive {
        return IfSetTabActive {
            tab,
        }
    }
}

impl MessageEncoder for IfSetTabActive {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETTAB_ACTIVE as i32;
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