use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateInvStopTransmit {
    component: i32,
}

impl UpdateInvStopTransmit {
    #[inline]
    pub fn new(
        component: i32,
    ) -> UpdateInvStopTransmit {
        return UpdateInvStopTransmit {
            component,
        }
    }
}

impl MessageEncoder for UpdateInvStopTransmit {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_INV_STOP_TRANSMIT as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}