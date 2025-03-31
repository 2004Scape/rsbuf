use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct CamReset {}

impl CamReset {
    pub fn new() -> CamReset {
        return CamReset {}
    }
}

impl MessageEncoder for CamReset {
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_RESET as i32;
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