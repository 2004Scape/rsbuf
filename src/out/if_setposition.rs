use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetPosition {
    component: i32,
    x: i32,
    y: i32,
}

impl IfSetPosition {
    #[inline]
    pub const fn new(
        component: i32,
        x: i32,
        y: i32,
    ) -> IfSetPosition {
        return IfSetPosition {
            component,
            x,
            y,
        }
    }
}

impl MessageEncoder for IfSetPosition {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETPOSITION as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 6;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.x);
        buf.p2(self.y);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }
}