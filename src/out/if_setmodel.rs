use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetModel {
    component: i32,
    model: i32,
}

impl IfSetModel {
    #[inline]
    pub fn new(
        component: i32,
        model: i32,
    ) -> IfSetModel {
        return IfSetModel {
            component,
            model,
        }
    }
}

impl MessageEncoder for IfSetModel {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETMODEL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.model);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}