use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct UpdatePid {
    pid: i32,
}

impl UpdatePid {
    #[inline]
    pub fn new(
        pid: i32,
    ) -> UpdatePid {
        return UpdatePid {
            pid,
        }
    }
}

impl MessageEncoder for UpdatePid {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_PID as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.pid);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}