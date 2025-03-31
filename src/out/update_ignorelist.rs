use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateIgnoreList {
    names: Vec<i64>,
}

impl UpdateIgnoreList {
    pub fn new(names: Vec<i64>) -> UpdateIgnoreList {
        return UpdateIgnoreList {
            names,
        }
    }
}

impl MessageEncoder for UpdateIgnoreList {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_IGNORELIST as i32;
    }

    fn length(&self) -> i32 {
        return -2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        for &name in self.names.iter() {
            buf.p8(name);
        }
    }

    fn test(&self) -> usize {
        return 8 * self.names.len();
    }
}