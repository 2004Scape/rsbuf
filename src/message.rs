use crate::packet::Packet;

pub trait InfoMessage {
    fn encode(&self, buf: &mut Packet);
    fn test(&self) -> usize;
    fn persists(&self) -> bool;
}

// ----

pub struct PlayerInfoAppearance {
    pub bytes: Vec<u8>,
}

impl PlayerInfoAppearance {
    #[inline]
    pub fn new(bytes: Vec<u8>) -> PlayerInfoAppearance {
        return PlayerInfoAppearance {
            bytes,
        };
    }
}

impl InfoMessage for PlayerInfoAppearance {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.bytes.len() as i32);
        buf.pdata(&self.bytes, 0, self.bytes.len());
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + self.bytes.len();
    }

    #[inline]
    fn persists(&self) -> bool {
        return true;
    }
}

// ----

pub struct PlayerInfoFaceEntity {
    pub entity: i32,
}

impl PlayerInfoFaceEntity {
    #[inline]
    pub fn new(entity: i32) -> PlayerInfoFaceEntity {
        return PlayerInfoFaceEntity {
            entity,
        }
    }
}

impl InfoMessage for PlayerInfoFaceEntity {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.entity);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoFaceCoord {
    pub x: i32,
    pub z: i32,
}

impl PlayerInfoFaceCoord {
    #[inline]
    pub fn new(x: i32, z: i32) -> PlayerInfoFaceCoord {
        return PlayerInfoFaceCoord {
            x,
            z,
        }
    }
}

impl InfoMessage for PlayerInfoFaceCoord {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.x);
        buf.p2(self.z);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoAnim {
    pub anim: i32,
    pub delay: i32,
}

impl PlayerInfoAnim {
    #[inline]
    pub fn new(anim: i32, delay: i32) -> PlayerInfoAnim {
        return PlayerInfoAnim {
            anim,
            delay,
        }
    }
}

impl InfoMessage for PlayerInfoAnim {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.anim);
        buf.p1(self.delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----