use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct CamShake {
    shake: i32,
    jitter: i32,
    amplitude: i32,
    frequency: i32,
}

impl CamShake {
    #[inline]
    pub fn new(
        shake: i32,
        jitter: i32,
        amplitude: i32,
        frequency: i32,
    ) -> CamShake {
        return CamShake {
            shake,
            jitter,
            amplitude,
            frequency,
        }
    }
}

impl MessageEncoder for CamShake {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::CAM_SHAKE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.shake); // direction?
        buf.p1(self.jitter);
        buf.p1(self.amplitude);
        buf.p1(self.frequency);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}