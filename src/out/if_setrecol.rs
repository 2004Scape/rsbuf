use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetRecol {
    component: i32,
    src: i32,
    dst: i32,
}

impl IfSetRecol {
    #[inline]
    pub fn new(
        component: i32,
        src: i32,
        dst: i32,
    ) -> IfSetRecol {
        return IfSetRecol {
            component,
            src,
            dst,
        }
    }
}

impl MessageEncoder for IfSetRecol {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETRECOL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 6;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.src);
        buf.p2(self.dst);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }
}