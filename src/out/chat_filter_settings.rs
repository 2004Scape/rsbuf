use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct ChatFilterSettings {
    public: i32,
    private: i32,
    trade: i32,
}

impl ChatFilterSettings {
    #[inline]
    pub const fn new(
        public: i32,
        private: i32,
        trade: i32,
    ) -> ChatFilterSettings {
        return ChatFilterSettings {
            public,
            private,
            trade,
        }
    }
}

impl MessageEncoder for ChatFilterSettings {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::CHAT_FILTER_SETTINGS as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 3;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.public);
        buf.p1(self.private);
        buf.p1(self.trade);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }
}