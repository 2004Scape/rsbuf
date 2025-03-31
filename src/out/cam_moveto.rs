use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct CamMoveTo {
    x: i32,
    z: i32,
    height: i32,
    speed: i32,
    multiplier: i32,
}

impl CamMoveTo {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_MOVETO as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.x);
        buf.p1(self.z);
        buf.p2(self.height);
        buf.p1(self.speed);
        buf.p1(self.multiplier);
    }

    fn test(&self) -> usize {
        return 6;
    }
}