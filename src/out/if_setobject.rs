use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct IfSetObject {
    component: i32,
    obj: i32,
    scale: i32,
}

impl IfSetObject {
    pub fn new(
        component: i32,
        obj: i32,
        scale: i32,
    ) -> IfSetObject {
        return IfSetObject {
            component,
            obj,
            scale,
        }
    }
}

impl MessageEncoder for IfSetObject {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_SETOBJECT as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
        buf.p2(self.obj);
        buf.p2(self.scale);
    }

    fn test(&self) -> usize {
        return 6;
    }
}