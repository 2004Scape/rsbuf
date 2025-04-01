use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct VarpLarge {
    varp: i32,
    value: i32,
}

impl VarpLarge {
    #[inline]
    pub const fn new(
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
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::VARP_LARGE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 6;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.varp);
        buf.p4(self.value);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }
}