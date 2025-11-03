use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct VarpSmall {
    varp: i32,
    value: i32,
}

impl VarpSmall {
    #[inline]
    pub const fn new(
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
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::VARP_SMALL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 3;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.varp);
        buf.p1(self.value);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }
}