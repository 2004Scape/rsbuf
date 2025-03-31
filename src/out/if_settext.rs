use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetText {
    component: i32,
    text: String,
}

impl IfSetText {
    pub fn new(
        component: i32,
        text: String,
    ) -> IfSetText {
        return IfSetText {
            component,
            text,
        }
    }
}

impl MessageEncoder for IfSetText {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETTEXT as i32;
    }

    fn length(&self) -> i32 {
        return -2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.pjstr(&self.text, 10);
    }

    fn test(&self) -> usize {
        return 3 + self.text.len();
    }
}