use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct VarpLarge {
    varp: i32,
    value: i32,
}

impl VarpLarge {
    pub fn new(
        varp: i32,
        value: i32,
    ) -> VarpLarge {
        return VarpLarge {
            varp,
            value,
        };
    }
}

impl MessageEncoder for VarpLarge {
    fn id(&self) -> i32 {
        return ServerInternalProt::VARP_LARGE as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.varp);
        buf.p4(self.value);
    }

    fn test(&self) -> usize {
        return 6;
    }
}