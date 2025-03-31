use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdateStat {
    stat: i32,
    experience: i32,
    level: i32,
}

impl UpdateStat {
    #[inline]
    pub fn new(
        stat: i32,
        experience: i32,
        level: i32,
    ) -> UpdateStat {
        return UpdateStat {
            stat,
            experience,
            level,
        }
    }
}

impl MessageEncoder for UpdateStat {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_STAT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 6;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.stat);
        buf.p4(self.experience / 10);
        buf.p1(self.level); // not base level
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }
}