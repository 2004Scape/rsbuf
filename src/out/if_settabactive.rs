use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetTabActive {
    tab: i32,
}

impl IfSetTabActive {
    #[inline]
    pub fn new(
        tab: i32,
    ) -> IfSetTabActive {
        return IfSetTabActive {
            tab,
        }
    }
}

impl MessageEncoder for IfSetTabActive {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETTAB_ACTIVE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 1;
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