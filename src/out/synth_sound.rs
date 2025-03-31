use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct SynthSound {
    synth: i32,
    loops: i32,
    delay: i32,
}

impl SynthSound {
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
    fn id(&self) -> i32 {
        return ServerInternalProt::SYNTH_SOUND as i32;
    }

    fn length(&self) -> i32 {
        return 5;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.synth);
        buf.p1(self.loops);
        buf.p2(self.delay);
    }

    fn test(&self) -> usize {
        return 5;
    }
}