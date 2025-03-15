use num_traits::real::Real;

#[derive(Clone)]
pub struct CoordGrid {
    pub coord: u32,
}

impl CoordGrid {
    #[inline]
    pub fn new(coord: u32) -> CoordGrid {
        return CoordGrid { coord };
    }

    #[inline]
    pub fn from(x: u16, y: u8, z: u16) -> CoordGrid {
        return CoordGrid {
            coord: ((z & 0x3fff) as u32)
                | (((x & 0x3fff) as u32) << 14)
                | (((y & 0x3) as u32) << 28),
        };
    }

    #[inline]
    pub fn y(&self) -> u8 {
        return ((self.coord >> 28) & 0x3) as u8;
    }

    #[inline]
    pub fn x(&self) -> u16 {
        return ((self.coord >> 14) & 0x3fff) as u16;
    }

    #[inline]
    pub fn z(&self) -> u16 {
        return (self.coord & 0x3fff) as u16;
    }

    #[inline]
    pub fn within_distance_sw(&self, other: &CoordGrid, distance: u8) -> bool {
        return !((self.x() as i32 - other.x() as i32).abs() > distance as i32 || (self.z() as i32 - other.z() as i32).abs() > distance as i32)
    }

    #[inline]
    pub fn fine(pos: u16, size: i32) -> i32 {
        return pos as i32 * 2 + size;
    }
}