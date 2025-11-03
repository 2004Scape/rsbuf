use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfSetText {
    component: i32,
    text: String,
}

impl IfSetText {
    #[inline]
    pub const fn new(
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
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETTEXT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.pjstr(&self.text, 10);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3 + self.text.len();
    }
}