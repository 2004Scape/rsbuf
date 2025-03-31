use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct IfOpenMainSide {
    main: i32,
    side: i32,
}

impl IfOpenMainSide {
    #[inline]
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
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENMAIN_SIDE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 4;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.main);
        buf.p2(self.side);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }
}