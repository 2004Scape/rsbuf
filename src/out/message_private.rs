use crate::message::MessageEncoder;
use crate::pack::WordPack;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MessagePrivateOut {
    from: i64,
    id: i32,
    staff_mod_level: i32,
    msg: String,
}

impl MessagePrivateOut {
    #[inline]
    pub fn new(
        from: i64,
        id: i32,
        staff_mod_level: i32,
        msg: String,
    ) -> MessagePrivateOut {
        return MessagePrivateOut {
            from,
            id,
            staff_mod_level,
            msg,
        }
    }
}

impl MessageEncoder for MessagePrivateOut {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::MESSAGE_PRIVATE as i32
    }

    #[inline]
    fn length(&self) -> i32 {
        return -1;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        let mut staff_mod_level = self.staff_mod_level;
        if staff_mod_level > 0 {
            staff_mod_level += 1;
        }

        buf.p8(self.from);
        buf.p4(self.id);
        buf.p1(staff_mod_level);
        let bytes: Vec<u8> = unsafe { WordPack::new().pack(self.msg.clone()) };
        buf.pdata(&bytes, 0, bytes.len())
    }

    #[inline]
    fn test(&self) -> usize {
        return 14 + self.msg.len();
    }
}