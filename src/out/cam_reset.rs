use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct CamReset {}

impl CamReset {
    #[inline]
    pub fn new() -> CamReset {
        return CamReset {}
    }
}

impl MessageEncoder for CamReset {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_RESET as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 0;
    }

    #[inline]
    fn encode(&self, _: &mut Packet) {}

    #[inline]
    fn test(&self) -> usize {
        return 0;
    }
}