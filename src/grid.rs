use nohash_hasher::{BuildNoHashHasher, IntMap, IntSet};
use std::collections::HashMap;

pub struct Zone {
    pub players: IntSet<i32>, // pids
    pub npcs: IntSet<i32>, // nids
}

impl Zone {
    #[inline]
    pub fn new() -> Zone {
        return Zone {
            players: IntSet::default(),
            npcs: IntSet::default(),
        }
    }

    #[inline]
    pub fn add_player(&mut self, player: i32) {
        self.players.insert(player);
    }

    #[inline]
    pub fn remove_player(&mut self, player: i32) {
        self.players.remove(&player);
    }

    #[inline]
    pub fn add_npc(&mut self, npc: i32) {
        self.npcs.insert(npc);
    }

    #[inline]
    pub fn remove_npc(&mut self, npc: i32) {
        self.npcs.remove(&npc);
    }
}

pub struct ZoneMap {
    pub zones: IntMap<u32, Zone>,
}

impl ZoneMap {
    #[inline]
    pub fn new() -> ZoneMap {
        let mut zones: HashMap<u32, Zone, BuildNoHashHasher<u32>> = IntMap::default();
        zones.reserve(0xffffff);
        return ZoneMap {
            zones,
        }
    }

    #[inline]
    pub const fn zone_index(x: u16, y: u8, z: u16) -> u32 {
        return (((x >> 3) & 0x7ff) as u32)
            | ((((z >> 3) & 0x7ff) as u32) << 11)
            | (((y & 0x3) as u32) << 22);
    }

    #[inline]
    pub const fn unpack_index(index: u32) -> (u16, u8, u16) {
        let x: u16 = ((index & 0x7ff) << 3) as u16;
        let z: u16 = (((index >> 11) & 0x7ff) << 3) as u16;
        let y: u8 = (index >> 22) as u8;
        return (x, y, z);
    }

    #[inline]
    pub fn zone(&mut self, x: u16, y: u8, z: u16) -> &mut Zone {
        return self
            .zones
            .entry(ZoneMap::zone_index(x, y, z))
            .or_insert(Zone::new());
    }
}