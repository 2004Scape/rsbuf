use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfOpenMainSide {
    main: i32,
    side: i32,
}

impl IfOpenMainSide {
    pub fn new(
        main: i32,
        side: i32,
    ) -> IfOpenMainSide {
        return IfOpenMainSide {
            main,
            side,
        }
    }
}

impl MessageEncoder for IfOpenMainSide {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENMAIN_SIDE as i32;
    }

    fn length(&self) -> i32 {
        return 4;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.main);
        buf.p2(self.side);
    }

    fn test(&self) -> usize {
        return 4;
    }
}