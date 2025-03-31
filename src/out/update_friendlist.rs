use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateFriendList {
    name: i64,
    node: i32,
}

impl UpdateFriendList {
    #[inline]
    pub fn new(name: i64, node: i32) -> UpdateFriendList {
        return UpdateFriendList {
            name,
            node,
        }
    }
}

impl MessageEncoder for UpdateFriendList {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_FRIENDLIST as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 9;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p8(self.name);
        buf.p1(self.node);
    }

    #[inline]
    fn test(&self) -> usize {
        return 9;
    }
}