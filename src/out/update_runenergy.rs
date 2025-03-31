use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateRunEnergy {
    energy: i32,
}

impl UpdateRunEnergy {
    pub fn new(
        energy: i32,
    ) -> UpdateRunEnergy {
        return UpdateRunEnergy {
            energy,
        }
    }
}

impl MessageEncoder for UpdateRunEnergy {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_RUNENERGY as i32;
    }

    fn length(&self) -> i32 {
        return 1;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.energy / 100);
    }

    fn test(&self) -> usize {
        return 1;
    }
}