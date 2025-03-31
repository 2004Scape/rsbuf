use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateRunWeight {
    kg: i32,
}

impl UpdateRunWeight {
    pub fn new(
        kg: i32,
    ) -> UpdateRunWeight {
        return UpdateRunWeight {
            kg,
        }
    }
}

impl MessageEncoder for UpdateRunWeight {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_RUNWEIGHT as i32;
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.kg);
    }

    fn test(&self) -> usize {
        return 2;
    }
}