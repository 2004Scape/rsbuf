use crate::coord::CoordGrid;
use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct UpdateZonePartialEnclosed {
    x: i32,
    z: i32,
    origin_x: i32,
    origin_z: i32,
    data: Vec<u8>,
}

impl UpdateZonePartialEnclosed {
    pub fn new(
        x: i32,
        z: i32,
        origin_x: i32,
        origin_z: i32,
        data: Vec<u8>,
    ) -> UpdateZonePartialEnclosed {
        return UpdateZonePartialEnclosed {
            x,
            z,
            origin_x,
            origin_z,
            data,
        }
    }
}

impl MessageEncoder for UpdateZonePartialEnclosed {
    fn id(&self) -> i32 {
        return ServerInternalProt::UPDATE_ZONE_PARTIAL_ENCLOSED as i32;
    }

    fn length(&self) -> i32 {
        return -2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1((self.x << 3) - CoordGrid::origin(self.origin_x as u16) as i32);
        buf.p1((self.z << 3) - CoordGrid::origin(self.origin_z as u16) as i32);
        buf.pdata(&self.data, 0, self.data.len());
    }

    fn test(&self) -> usize {
        return 2 + self.data.len();
    }
}