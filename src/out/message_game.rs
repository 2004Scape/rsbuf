use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MessageGame {
    msg: String,
}

impl MessageGame {
    pub fn new(msg: String) -> MessageGame {
        return MessageGame {
            msg,
        }
    }
}

impl MessageEncoder for MessageGame {
    fn id(&self) -> i32 {
        return ServerInternalProt::MESSAGE_GAME as i32
    }

    fn length(&self) -> i32 {
        return -1;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.msg, 10);
    }

    fn test(&self) -> usize {
        return 1 + self.msg.len();
    }
}