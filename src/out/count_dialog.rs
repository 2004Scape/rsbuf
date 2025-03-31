use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct PCountDialog {}

impl PCountDialog {
    pub fn new() -> PCountDialog {
        return PCountDialog {}
    }
}

impl MessageEncoder for PCountDialog {
    fn id(&self) -> i32 {
        return ServerInternalProt::P_COUNTDIALOG as i32;
    }

    fn length(&self) -> i32 {
        return 0;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, _: &mut Packet) {}

    fn test(&self) -> usize {
        return 0;
    }
}