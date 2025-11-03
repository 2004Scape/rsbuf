use crate::coord::CoordGrid;
use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct LocMerge {
    coord: i32,
    src_x: i32,
    src_z: i32,
    shape: i32,
    angle: i32,
    loc: i32,
    start: i32,
    end: i32,
    pid: i32,
    east: i32,
    south: i32,
    west: i32,
    north: i32,
}

impl LocMerge {
    #[inline]
    pub const fn new(
        src_x: i32,
        src_z: i32,
        shape: i32,
        angle: i32,
        loc: i32,
        start: i32,
        end: i32,
        pid: i32,
        east: i32,
        south: i32,
        west: i32,
        north: i32,
    ) -> LocMerge {
        return LocMerge {
            coord: CoordGrid::pack_zone_coord(src_x as u16, src_z as u16) as i32,
            src_x,
            src_z,
            shape,
            angle,
            loc,
            start,
            end,
            pid,
            east,
            south,
            west,
            north,
        }
    }
}

impl MessageEncoder for LocMerge {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::LOC_MERGE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 14;
    }
    #[inline]

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord);
        buf.p1((self.shape << 2) | (self.angle & 0x3));
        buf.p2(self.loc);
        buf.p2(self.start);
        buf.p2(self.end);
        buf.p2(self.pid);
        buf.p1(self.east - self.src_x);
        buf.p1(self.south - self.src_z);
        buf.p1(self.west - self.src_x);
        buf.p1(self.north - self.src_z);
    }

    #[inline]
    fn test(&self) -> usize {
        return 14;
    }
}