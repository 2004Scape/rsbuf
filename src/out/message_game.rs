use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct MessageGame {
    msg: String,
}

impl MessageGame {
    #[inline]
    pub fn new(msg: String) -> MessageGame {
        return MessageGame {
            msg,
        }
    }
}

impl MessageEncoder for MessageGame {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::MESSAGE_GAME as i32
    }

    #[inline]
    fn length(&self) -> i32 {
        return -1;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.msg, 10);
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + self.msg.len();
    }
}