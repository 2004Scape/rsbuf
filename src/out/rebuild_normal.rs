use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct RebuildNormal {
    x: i32,
    z: i32,
    squares: Vec<u16>,
    maps: Vec<i32>,
    locs: Vec<i32>,
}

impl RebuildNormal {
    #[inline]
    pub const fn new(
        x: i32,
        z: i32,
        squares: Vec<u16>,
        maps: Vec<i32>,
        locs: Vec<i32>,
    ) -> RebuildNormal {
        return RebuildNormal {
            x,
            z,
            squares,
            maps,
            locs,
        }
    }
}

impl MessageEncoder for RebuildNormal {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::REBUILD_NORMAL as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.x);
        buf.p2(self.z);
        for index in 0..self.squares.len() {
            buf.p2(unsafe { *self.squares.as_ptr().add(index) as i32 });
            buf.p4(unsafe { *self.maps.as_ptr().add(index) });
            buf.p4(unsafe { *self.locs.as_ptr().add(index) });
        }
    }

    #[inline]
    fn test(&self) -> usize {
        return 2 + 2 + self.squares.len() * (1 + 1 + 4 + 4);
    }
}