use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct ChatFilterSettings {
    public: i32,
    private: i32,
    trade: i32,
}

impl ChatFilterSettings {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::CHAT_FILTER_SETTINGS as i32;
    }

    fn length(&self) -> i32 {
        return 3;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.public);
        buf.p1(self.private);
        buf.p1(self.trade);
    }

    fn test(&self) -> usize {
        return 3;
    }
}