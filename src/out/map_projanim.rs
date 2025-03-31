use crate::coord::CoordGrid;
use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct MapProjAnim {
    coord: i32,
    src_x: i32,
    src_z: i32,
    dst_x: i32,
    dst_z: i32,
    target: i32,
    spotanim: i32,
    src_height: i32,
    dst_height: i32,
    start: i32,
    end: i32,
    peak: i32,
    arc: i32,
}

impl MapProjAnim {
    pub fn new(
        src_x: i32,
        src_z: i32,
        dst_x: i32,
        dst_z: i32,
        target: i32,
        spotanim: i32,
        src_height: i32,
        dst_height: i32,
        start: i32,
        end: i32,
        peak: i32,
        arc: i32,
    ) -> MapProjAnim {
        return MapProjAnim {
            coord: CoordGrid::pack_zone_coord(src_x as u16, src_z as u16) as i32,
            src_x,
            src_z,
            dst_x,
            dst_z,
            target,
            spotanim,
            src_height,
            dst_height,
            start,
            end,
            peak,
            arc,
        }
    }
}

impl MessageEncoder for MapProjAnim {
    fn id(&self) -> i32 {
        return ServerInternalProt::MAP_PROJANIM as i32;
    }

    fn length(&self) -> i32 {
        return 15;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p1(self.dst_x - self.src_x);
        buf.p1(self.dst_z - self.src_z);
        buf.p2(self.target); // 0: coord, > 0: npc, < 0: player
        buf.p2(self.spotanim);
        buf.p1(self.src_height);
        buf.p1(self.dst_height);
        buf.p2(self.start);
        buf.p2(self.end);
        buf.p1(self.peak);
        buf.p1(self.arc);
    }

    fn test(&self) -> usize {
        return 15;
    }
}