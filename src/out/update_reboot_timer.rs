use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateRebootTimer {
    ticks: i32,
}

impl UpdateRebootTimer {
    #[inline]
    pub fn new(
        ticks: i32,
    ) -> UpdateRebootTimer {
        return UpdateRebootTimer {
            ticks,
        }
    }
}

impl MessageEncoder for UpdateRebootTimer {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_REBOOT_TIMER as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered; // todo: what should priority be?
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.ticks);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}