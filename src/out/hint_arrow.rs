use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct HintArrow {
    arrow: i32,
    nid: i32,
    pid: i32,
    x: i32,
    z: i32,
    y: i32,
}

impl HintArrow {
    pub fn new(
        arrow: i32,
        nid: i32,
        pid: i32,
        x: i32,
        z: i32,
        y: i32,
    ) -> HintArrow {
        return HintArrow {
            arrow,
            nid,
            pid,
            x,
            z,
            y,
        }
    }
}

impl MessageEncoder for HintArrow {
    fn id(&self) -> i32 {
        return ServerInternalProt::HINT_ARROW as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered; // todo: what should priority be?
    }

    fn encode(&self, buf: &mut Packet) {
        match self.arrow {
            1 => {
                buf.p1(self.arrow);
                buf.p2(self.nid);
                buf.p2(0);
                buf.p1(0);
            },
            2..=6 => {
                // 2 - 64, 64 offset - centered
                // 3 - 0, 64 offset - far left
                // 4 - 128, 64 offset - far right
                // 5 - 64, 0 offset - bottom left
                // 6 - 64, 128 offset - top left
                buf.p1(self.arrow);
                buf.p2(self.x);
                buf.p2(self.z);
                buf.p1(self.y);
            },
            10 => {
                buf.p1(self.arrow);
                buf.p2(self.pid);
                buf.p2(0);
                buf.p1(0);
            },
            _ => {
                buf.p1(-1);
                buf.p2(0);
                buf.p2(0);
                buf.p1(0);
            },
        }
    }

    fn test(&self) -> usize {
        return 6;
    }
}