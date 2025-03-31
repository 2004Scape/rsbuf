use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdatePid {
    pid: i32,
}

impl UpdatePid {
    pub fn new(
        pid: i32,
    ) -> UpdatePid {
        return UpdatePid {
            pid,
        }
    }
}

impl MessageEncoder for UpdatePid {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_PID as i32;
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate; // todo: what should priority be?
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.pid);
    }

    fn test(&self) -> usize {
        return 2;
    }
}