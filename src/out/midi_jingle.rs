use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct MidiJingle {
    delay: i32,
    data: Vec<u8>,
}

impl MidiJingle {
    #[inline]
    pub const fn new(
        delay: i32,
        data: Vec<u8>,
    ) -> MidiJingle {
        return MidiJingle {
            delay,
            data,
        }
    }
}

impl MessageEncoder for MidiJingle {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::MIDI_JINGLE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.delay);
        buf.pdata(&self.data, 0, self.data.len());
    }

    #[inline]
    fn test(&self) -> usize {
        return 6 + self.data.len();
    }
}