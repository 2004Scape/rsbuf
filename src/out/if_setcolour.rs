use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetColour {
    component: i32,
    colour: i32,
}

impl IfSetColour {
    #[inline]
    pub const fn new(
        component: i32,
        colour: i32,
    ) -> IfSetColour {
        return IfSetColour {
            component,
            colour,
        }
    }
}

impl MessageEncoder for IfSetColour {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETCOLOUR as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.colour);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}