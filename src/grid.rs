use std::collections::HashMap;

pub struct Zone {
    pub players: Vec<i32>, // pids
    pub npcs: Vec<i32>, // nids
}

impl Zone {
    #[inline]
    pub fn new() -> Zone {
        return Zone {
            players: Vec::new(),
            npcs: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: i32) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: i32) {
        self.players.retain(|&pid| pid != player);
    }

    pub fn add_npc(&mut self, npc: i32) {
        self.npcs.push(npc);
    }

    pub fn remove_npc(&mut self, npc: i32) {
        self.npcs.retain(|&nid| nid != npc);
    }
}

pub struct ZoneMap {
    pub zones: HashMap<u32, Zone>,
}

impl ZoneMap {
    #[inline]
    pub fn new() -> ZoneMap {
        return ZoneMap {
            zones: HashMap::with_capacity(0xffffff),
        }
    }

    #[inline]
    pub fn zone_index(x: u16, y: u8, z: u16) -> u32 {
        return (((x >> 3) & 0x7ff) as u32)
            | ((((z >> 3) & 0x7ff) as u32) << 11)
            | (((y & 0x3) as u32) << 22);
    }

    #[inline]
    pub fn unpack_index(index: u32) -> (u16, u8, u16) {
        let x: u16 = ((index & 0x7ff) << 3) as u16;
        let z: u16 = (((index >> 11) & 0x7ff) << 3) as u16;
        let y: u8 = (index >> 22) as u8;
        return (x, y, z);
    }

    #[inline]
    pub fn zone(&mut self, x: u16, y: u8, z: u16) -> &mut Zone {
        let zone_index: u32 = ZoneMap::zone_index(x, y, z);
        return self
            .zones
            .entry(zone_index)
            .or_insert(Zone::new());
    }
}