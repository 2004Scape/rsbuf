use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct SynthSound {
    synth: i32,
    loops: i32,
    delay: i32,
}

impl SynthSound {
    #[inline]
    pub fn new(
        synth: i32,
        loops: i32,
        delay: i32,
    ) -> SynthSound {
        return SynthSound {
            synth,
            loops,
            delay,
        }
    }
}

impl MessageEncoder for SynthSound {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::SYNTH_SOUND as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 5;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.synth);
        buf.p1(self.loops);
        buf.p2(self.delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 5;
    }
}