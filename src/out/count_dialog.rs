use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct PCountDialog {}

impl PCountDialog {
    #[inline]
    pub fn new() -> PCountDialog {
        return PCountDialog {}
    }
}

impl MessageEncoder for PCountDialog {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::P_COUNTDIALOG as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}