use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct LastLoginInfo {
    last_ip: i32,
    days_since_login: i32,
    days_since_recovery: i32,
    messages: i32,
}

impl LastLoginInfo {
    #[inline]
    pub const fn new(
        last_ip: i32,
        days_since_login: i32,
        days_since_recovery: i32,
        messages: i32,
    ) -> LastLoginInfo {
        return LastLoginInfo {
            last_ip,
            days_since_login,
            days_since_recovery,
            messages,
        }
    }
}

impl MessageEncoder for LastLoginInfo {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::LAST_LOGIN_INFO as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 9;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p4(self.last_ip);
        buf.p2(self.days_since_login);
        buf.p1(self.days_since_recovery);
        buf.p2(self.messages);
    }

    #[inline]
    fn test(&self) -> usize {
        return 9;
    }
}