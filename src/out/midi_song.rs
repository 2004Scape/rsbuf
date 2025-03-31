use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MidiSong {
    name: String,
    crc: i32,
    length: i32,
}

impl MidiSong {
    #[inline]
    pub fn new(
        name: String,
        crc: i32,
        length: i32,
    ) -> MidiSong {
        return MidiSong {
            name,
            crc,
            length,
        }
    }
}

impl MessageEncoder for MidiSong {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::MIDI_SONG as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -1;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.name, 10);
        buf.p4(self.crc);
        buf.p4(self.length);
    }

    #[inline]
    fn test(&self) -> usize {
        return 9 + self.name.len();
    }
}