use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MidiJingle {
    delay: i32,
    data: Vec<u8>,
}

impl MidiJingle {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::MIDI_JINGLE as i32;
    }

    fn length(&self) -> i32 {
        return -2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.delay);
        buf.pdata(&self.data, 0, self.data.len());
    }

    fn test(&self) -> usize {
        return 6 + self.data.len();
    }
}