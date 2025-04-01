use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct CamMoveTo {
    x: i32,
    z: i32,
    height: i32,
    speed: i32,
    multiplier: i32,
}

impl CamMoveTo {
    #[inline]
    pub const fn new(
        x: i32,
        z: i32,
        height: i32,
        speed: i32,
        multiplier: i32,
    ) -> CamMoveTo {
        return CamMoveTo {
            x,
            z,
            height,
            speed,
            multiplier,
        }
    }
}

impl MessageEncoder for CamMoveTo {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_MOVETO as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 6;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.x);
        buf.p1(self.z);
        buf.p2(self.height);
        buf.p1(self.speed);
        buf.p1(self.multiplier);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }
}