use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct VarpSmall {
    varp: i32,
    value: i32,
}

impl VarpSmall {
    pub fn new(
        varp: i32,
        value: i32,
    ) -> VarpSmall {
        return VarpSmall {
            varp,
            value,
        };
    }
}

impl MessageEncoder for VarpSmall {
    fn id(&self) -> i32 {
        return ServerInternalProt::VARP_SMALL as i32;
    }

    fn length(&self) -> i32 {
        return 3;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.varp);
        buf.p1(self.value);
    }

    fn test(&self) -> usize {
        return 3;
    }
}