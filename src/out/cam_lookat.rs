use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct CamLookAt {
    x: i32,
    z: i32,
    height: i32,
    speed: i32,
    multiplier: i32,
}

impl CamLookAt {
    pub fn new(
        x: i32,
        z: i32,
        height: i32,
        speed: i32,
        multiplier: i32,
    ) -> CamLookAt {
        return CamLookAt {
            x,
            z,
            height,
            speed,
            multiplier,
        }
    }
}

impl MessageEncoder for CamLookAt {
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_LOOKAT as i32;
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