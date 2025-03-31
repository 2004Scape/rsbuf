use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct RebuildNormal {
    x: i32,
    z: i32,
    squares: Vec<u16>,
    maps: Vec<i32>,
    locs: Vec<i32>,
}

impl RebuildNormal {
    pub fn new(
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
    fn id(&self) -> i32 {
        return ServerInternalProt::REBUILD_NORMAL as i32;
    }

    fn length(&self) -> i32 {
        return -2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.x);
        buf.p2(self.z);
        for index in 0..self.squares.len() {
            let mapsquare: u16 = unsafe { *self.squares.as_ptr().add(index) };
            buf.p1(((mapsquare >> 8) & 0xff) as i32);
            buf.p1((mapsquare & 0xff) as i32);
            buf.p4(unsafe { *self.maps.as_ptr().add(index) });
            buf.p4(unsafe { *self.locs.as_ptr().add(index) });
        }
        // for &packed in self.squares.iter() {
        //     // buf.p1(((packed >> 72) & 0xff) as i32);
        //     // buf.p1(((packed >> 64) & 0xff) as i32);
        //     // buf.p4(((packed >> 32) & 0xffffffff) as i32);
        //     // buf.p4((packed & 0xffffffff) as i32);
        //     // (50n << 72n) + (50n << 64n) + (BigInt.asUintN(32, -2147483648n) << 32n) + 2147483647n
        // }
    }

    fn test(&self) -> usize {
        return 2 + 2 + self.squares.len() * (1 + 1 + 4 + 4);
    }
}