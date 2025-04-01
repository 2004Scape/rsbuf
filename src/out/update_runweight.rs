use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdateRunWeight {
    kg: i32,
}

impl UpdateRunWeight {
    #[inline]
    pub const fn new(
        kg: i32,
    ) -> UpdateRunWeight {
        return UpdateRunWeight {
            kg,
        }
    }
}

impl MessageEncoder for UpdateRunWeight {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_RUNWEIGHT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.kg);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}