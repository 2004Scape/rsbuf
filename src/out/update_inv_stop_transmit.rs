use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateInvStopTransmit {
    component: i32,
}

impl UpdateInvStopTransmit {
    pub fn new(
        component: i32,
    ) -> UpdateInvStopTransmit {
        return UpdateInvStopTransmit {
            component,
        }
    }
}

impl MessageEncoder for UpdateInvStopTransmit {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_INV_STOP_TRANSMIT as i32;
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    fn test(&self) -> usize {
        return 2;
    }
}