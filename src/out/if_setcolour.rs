use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetColour {
    component: i32,
    colour: i32,
}

impl IfSetColour {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETCOLOUR as i32;
    }

    fn length(&self) -> i32 {
        return 4;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.colour);
    }

    fn test(&self) -> usize {
        return 4;
    }
}