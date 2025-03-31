use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MapAnim {
    coord: i32,
    spotanim: i32,
    height: i32,
    delay: i32,
}

impl MapAnim {
    pub fn new(
        coord: i32,
        spotanim: i32,
        height: i32,
        delay: i32,
    ) -> MapAnim {
        return MapAnim {
            coord,
            spotanim,
            height,
            delay,
        }
    }
}

impl MessageEncoder for MapAnim {
    fn id(&self) -> i32 {
        return ServerInternalProt::MAP_ANIM as i32;
    }

    fn length(&self) -> i32 {
        return 6;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p2(self.spotanim);
        buf.p1(self.height);
        buf.p2(self.delay);
    }

    fn test(&self) -> usize {
        return 6;
    }
}