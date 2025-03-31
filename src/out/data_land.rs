use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::ServerInternalProt;

pub struct DataLand {
    x: i32,
    z: i32,
    offset: i32,
    length: i32,
    data: Vec<u8>,
}

impl DataLand {
    #[inline]
    pub fn new(
        x: i32,
        z: i32,
        offset: i32,
        length: i32,
        data: Vec<u8>,
    ) -> DataLand {
        return DataLand {
            x,
            z,
            offset,
            length,
            data,
        }
    }
}

impl MessageEncoder for DataLand {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::DATA_LAND as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
    }

    #[inline]
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.x);
        buf.p1(self.z);
        buf.p2(self.offset);
        buf.p2(self.length);
        buf.pdata(&self.data, 0, self.data.len());
    }

    #[inline]
    fn test(&self) -> usize {
        return 6 + self.data.len();
    }
}

// ----

pub struct DataLandDone {
    x: i32,
    z: i32,
}

impl DataLandDone {
    #[inline]
    pub fn new(
        x: i32,
        z: i32,
    ) -> DataLandDone {
        return DataLandDone {
            x,
            z,
        }
    }
}

impl MessageEncoder for DataLandDone {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::DATA_LAND_DONE as i32;
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
        buf.p1(self.x);
        buf.p1(self.z);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }
}
