use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateStat {
    stat: i32,
    experience: i32,
    level: i32,
}

impl UpdateStat {
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
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_STAT as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.stat);
        buf.p4(self.experience / 10);
        buf.p1(self.level); // not base level
    }

    fn test(&self) -> usize {
        return 6;
    }
}