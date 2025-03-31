use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetTab {
    component: i32,
    tab: i32,
}

impl IfSetTab {
    #[inline]
    pub fn new(
        component: i32,
        tab: i32,
    ) -> IfSetTab {
        return IfSetTab {
            component,
            tab,
        }
    }
}

impl MessageEncoder for IfSetTab {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETTAB as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 3;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p1(self.tab);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }
}