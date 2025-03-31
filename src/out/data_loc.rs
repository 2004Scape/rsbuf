use crate::message::MessageEncoder;
use crate::packet::Packet;
use crate::prot::ServerInternalProt;

pub struct DataLoc {
    x: i32,
    z: i32,
    offset: i32,
    length: i32,
    data: Vec<u8>,
}

impl DataLoc {
    #[inline]
    pub fn new(
        x: i32,
        z: i32,
        offset: i32,
        length: i32,
        data: Vec<u8>,
    ) -> DataLoc {
        return DataLoc {
            x,
            z,
            offset,
            length,
            data,
        }
    }
}

impl MessageEncoder for DataLoc {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::DATA_LOC as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return -2;
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

pub struct DataLocDone {
    x: i32,
    z: i32,
}

impl DataLocDone {
    #[inline]
    pub fn new(
        x: i32,
        z: i32,
    ) -> DataLocDone {
        return DataLocDone {
            x,
            z,
        }
    }
}

impl MessageEncoder for DataLocDone {
    #[inline]
    fn id(&self) -> i32 {
        return ServerInternalProt::DATA_LOC_DONE as i32;
    }

    #[inline]
    fn length(&self) -> i32 {
        return 2;
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
