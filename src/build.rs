use crate::coord::CoordGrid;
use crate::grid::ZoneMap;
use crate::npc::Npc;
use crate::player::Player;
use std::collections::HashMap;

#[derive(Clone)]
pub struct IdBitSet {
    bits: Vec<i32>,
    ids: Vec<i32>,
}

impl IdBitSet {
    #[inline]
    pub fn new(len: usize, capacity: usize) -> IdBitSet {
        return IdBitSet {
            bits: vec![0; len / 32],
            ids: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn contains(&self, id: i32) -> bool {
        return unsafe { *self.bits.as_ptr().add((id >> 5) as usize) & (1 << (id & 0x1f)) != 0 };
    }

    #[inline]
    pub fn insert(&mut self, id: i32) {
        if self.contains(id) {
            return;
        }
        unsafe { *self.bits.as_mut_ptr().add((id >> 5) as usize) |= 1 << (id & 0x1f) };
        self.ids.push(id);
    }

    #[inline]
    pub fn remove(&mut self, id: i32) {
        if !self.contains(id) {
            return;
        }
        unsafe { *self.bits.as_mut_ptr().add((id >> 5) as usize) &= !(1 << (id & 0x1f)) };
        if let Some(index) = self.ids.iter().position(|&x| x == id) {
            self.ids.remove(index);
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.ids.len();
    }

    #[inline]
    pub fn iter(&self) -> Vec<i32> {
        return self.ids.iter().cloned().collect();
    }

    #[inline]
    fn clear(&mut self) {
        self.bits.fill(0);
        self.ids.clear();
    }
}

#[derive(Clone)]
pub struct BuildArea {
    pub players: IdBitSet,
    pub npcs: IdBitSet,
    appearances: [u32; 2048],
    force_view_distance: bool,
    pub view_distance: u8,
    last_resize: u32,
}

impl BuildArea {
    pub const INTERVAL: u8 = 10;
    pub const PREFERRED_PLAYERS: u8 = 250;
    pub const PREFERRED_NPCS: u8 = 255;
    pub const PREFERRED_VIEW_DISTANCE: u8 = 15;

    #[inline]
    pub fn new() -> BuildArea {
        return BuildArea {
            players: IdBitSet::new(2048, BuildArea::PREFERRED_PLAYERS as usize), // 64 bitset
            npcs: IdBitSet::new(8192, BuildArea::PREFERRED_NPCS as usize), // 256 bitset
            appearances: [0; 2048],
            force_view_distance: false,
            view_distance: BuildArea::PREFERRED_VIEW_DISTANCE,
            last_resize: 0,
        }
    }

    #[inline]
    pub fn cleanup(&mut self) {
        self.players.clear();
        self.npcs.clear();
        self.appearances.fill(0);
    }

    #[inline]
    pub fn resize(&mut self) {
        if self.force_view_distance {
            return;
        }

        if self.players.len() >= BuildArea::PREFERRED_PLAYERS as usize {
            if self.view_distance > 1 {
                self.view_distance -= 1;
            }
            self.last_resize = 0;
            return;
        }

        self.last_resize += 1;
        if self.last_resize >= BuildArea::INTERVAL as u32 {
            if self.view_distance < BuildArea::PREFERRED_VIEW_DISTANCE {
                self.view_distance += 1;
            } else {
                self.last_resize = 0;
            }
        }
    }

    #[inline]
    pub fn rebuild_npcs(&mut self) {
        // optimization to avoid sending 3 bits * observed npcs when everything has to be removed anyways
        self.npcs.clear();
    }

    #[inline]
    pub fn rebuild_players(&mut self, players: &[Option<Player>], grid: &HashMap<u32, Vec<i32>>, pid: i32, x: u16, y: u8, z: u16) {
        // optimization to avoid sending 3 bits * observed players when everything has to be removed anyways
        self.players.clear();
        self.last_resize = 0;
        self.view_distance = BuildArea::PREFERRED_VIEW_DISTANCE;
        // pre calc if we can go ahead and shorten view distance
        let mut count: u8 = 0;
        let mut decrement: bool = false;
        for _ in self.get_nearby_players_nearest(players, grid, pid, x, y, z) {
            count += 1;
            if count >= BuildArea::PREFERRED_PLAYERS {
                decrement = true;
                break;
            }
        }
        if decrement {
            self.view_distance -= 1;
        }
    }

    #[inline]
    pub const fn has_appearance(&self, pid: i32, tick: u32) -> bool {
        return unsafe { *self.appearances.as_ptr().add(pid as usize) == tick }
    }

    #[inline]
    pub fn save_appearance(&mut self, pid: i32, tick: u32) {
        unsafe { *self.appearances.as_mut_ptr().add(pid as usize) = tick }
    }

    #[inline]
    pub fn get_nearby_players(
        &self,
        players: &[Option<Player>],
        grid: &HashMap<u32, Vec<i32>>,
        map: &mut ZoneMap,
        pid: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> Vec<i32> {
        return if self.view_distance < BuildArea::PREFERRED_VIEW_DISTANCE {
            self.get_nearby_players_nearest(players, grid, pid, x, y, z)
        } else {
            self.get_nearby_players_zones(players, map, pid, x, y, z)
        }
    }

    #[inline]
    pub fn get_nearby_players_zones(
        &self,
        players: &[Option<Player>],
        map: &mut ZoneMap,
        pid: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> Vec<i32> {
        let distance: u16 = self.view_distance as u16;
        let start_x: u16 = (x.saturating_sub(distance)) >> 3;
        let start_z: u16 = (z.saturating_sub(distance)) >> 3;
        let end_x: u16 = (x.saturating_add(distance)) >> 3;
        let end_z: u16 = (z.saturating_add(distance)) >> 3;

        let count: usize = self.players.len();
        let mut nearby: Vec<i32> = Vec::with_capacity(BuildArea::PREFERRED_PLAYERS as usize - count);

        for zx in start_x..=end_x {
            let zone_x: u16 = zx << 3;
            for zz in start_z..=end_z {
                if nearby.len() + count >= BuildArea::PREFERRED_PLAYERS as usize {
                    return nearby;
                }
                let zone_z: u16 = zz << 3;
                nearby.extend(
                    map.zone(zone_x, y, zone_z).players
                        .iter()
                        .take(BuildArea::PREFERRED_PLAYERS as usize - nearby.len())
                        .filter(|&&player| self.filter_player(players, player, pid, x, y, z)),
                );
            }
        }
        return nearby
    }

    #[inline]
    pub fn get_nearby_players_nearest(
        &self,
        players: &[Option<Player>],
        grid: &HashMap<u32, Vec<i32>>,
        pid: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> Vec<i32> {
        let radius: i32 = (self.view_distance as i32) * 2;
        let min: i32 = -(radius >> 1);
        let max: i32 = radius >> 1;
        let length: i32 = radius.pow(2);

        let (mut dx, mut dz): (i32, i32) = (0, 0);
        let (mut ldx, mut ldz): (i32, i32) = (0, -1);

        let count: usize = self.players.len();
        let mut nearby: Vec<i32> = Vec::with_capacity(BuildArea::PREFERRED_PLAYERS as usize - count);

        for _ in 1..=length {
            if nearby.len() + count >= BuildArea::PREFERRED_PLAYERS as usize {
                return nearby;
            }
            if (min < dx && dx <= max) && (min < dz && dz <= max) {
                if let Some(set) = grid.get(&CoordGrid::from(((x as i32) + dx) as u16, y, ((z as i32) + dz) as u16).packed) {
                    nearby.extend(
                        set
                            .iter()
                            .take(BuildArea::PREFERRED_PLAYERS as usize - nearby.len())
                            .filter(|&&player| self.filter_player(players, player, pid, x, y, z)),
                    );
                    if nearby.len() + count >= BuildArea::PREFERRED_PLAYERS as usize {
                        return nearby;
                    }
                }
            }
            if dx == dz || (dx < 0 && dx == -dz) || (dx > 0 && dx == 1 - dz) {
                (ldx, ldz) = (-ldz, ldx);
            }
            dx += ldx;
            dz += ldz;
        }
        return nearby
    }

    #[inline]
    pub fn get_nearby_npcs(
        &self,
        npcs: &[Option<Npc>],
        map: &mut ZoneMap,
        x: u16,
        y: u8,
        z: u16
    ) -> Vec<i32> {
        let distance: u16 = BuildArea::PREFERRED_VIEW_DISTANCE as u16;
        let start_x: u16 = (x.saturating_sub(distance)) >> 3;
        let start_z: u16 = (z.saturating_sub(distance)) >> 3;
        let end_x: u16 = (x.saturating_add(distance)) >> 3;
        let end_z: u16 = (z.saturating_add(distance)) >> 3;

        let count: usize = self.npcs.len();
        let mut nearby: Vec<i32> = Vec::with_capacity(BuildArea::PREFERRED_NPCS as usize - count);

        for zx in start_x..=end_x {
            let zone_x: u16 = zx << 3;
            for zz in start_z..=end_z {
                if nearby.len() + count >= BuildArea::PREFERRED_NPCS as usize {
                    return nearby;
                }
                let zone_z: u16 = zz << 3;
                nearby.extend(
                    map.zone(zone_x, y, zone_z).npcs
                        .iter()
                        .take(BuildArea::PREFERRED_NPCS as usize - nearby.len())
                        .filter(|&&npc| self.filter_npc(npcs, npc, x, y, z)),
                );
            }
        }
        return nearby
    }

    #[inline]
    fn filter_player(
        &self,
        players: &[Option<Player>],
        player: i32,
        pid: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> bool {
        return match unsafe { &*players.as_ptr().add(player as usize) } {
            None => false,
            Some(other) => !(self.players.contains(player) || !CoordGrid::within_distance_sw(&other.coord, &CoordGrid::from(x, y, z), self.view_distance) || other.pid == -1 || other.pid == pid || other.coord.y() != y),
        };
    }

    #[inline]
    fn filter_npc(
        &self,
        npcs: &[Option<Npc>],
        npc: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> bool {
        return match unsafe { &*npcs.as_ptr().add(npc as usize) } {
            None => false,
            Some(other) => !(self.npcs.contains(npc) || !CoordGrid::within_distance_sw(&other.coord, &CoordGrid::from(x, y, z), BuildArea::PREFERRED_VIEW_DISTANCE) || other.nid == -1 || other.coord.y() != y || !other.active),
        };
    }
}