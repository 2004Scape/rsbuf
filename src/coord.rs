#[derive(Clone)]
pub struct CoordGrid {
    pub packed: u32,
}

impl CoordGrid {
    #[inline]
    pub const fn new(packed: u32) -> CoordGrid {
        return CoordGrid { packed };
    }

    #[inline]
    pub const fn from(x: u16, y: u8, z: u16) -> CoordGrid {
        return CoordGrid {
            packed: ((z & 0x3fff) as u32)
                | (((x & 0x3fff) as u32) << 14)
                | (((y & 0x3) as u32) << 28),
        };
    }

    #[inline]
    pub const fn y(&self) -> u8 {
        return ((self.packed >> 28) & 0x3) as u8;
    }

    #[inline]
    pub const fn x(&self) -> u16 {
        return ((self.packed >> 14) & 0x3fff) as u16;
    }

    #[inline]
    pub const fn z(&self) -> u16 {
        return (self.packed & 0x3fff) as u16;
    }

    #[inline]
    pub const fn within_distance_sw(&self, other: &CoordGrid, distance: u8) -> bool {
        return !((self.x() as i32 - other.x() as i32).abs() > distance as i32 || (self.z() as i32 - other.z() as i32).abs() > distance as i32)
    }

    #[inline]
    pub const fn fine(pos: u16, size: i32) -> i32 {
        return pos as i32 * 2 + size;
    }

    #[inline]
    pub const fn zone(pos: u16) -> u16 {
        return pos >> 3;
    }
}