use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdateRunEnergy {
    energy: i32,
}

impl UpdateRunEnergy {
    #[inline]
    pub const fn new(
        energy: i32,
    ) -> UpdateRunEnergy {
        return UpdateRunEnergy {
            energy,
        }
    }
}

impl MessageEncoder for UpdateRunEnergy {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_RUNENERGY as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 1;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.energy / 100);
    }

    #[inline]
    fn test(&self) -> usize {
        return 1;
    }
}