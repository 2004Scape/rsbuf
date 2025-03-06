use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use web_sys::console;
use crate::coord::CoordGrid;
use crate::player::Player;

#[derive(Eq, PartialEq, Clone)]
pub struct BuildArea {
    pub players: Vec<i32>,
    pub npcs: HashSet<i32>,
    appearances: HashMap<i32, u32>,
    force_view_distance: bool,
    pub view_distance: u8,
    last_resize: u32,
}

impl BuildArea {
    pub const INTERVAL: u8 = 10;
    pub const PREFERRED_PLAYERS: u8 = 250;
    pub const PREFERRED_NPCS: u8 = 255;
    pub const PREFERRED_VIEW_DISTANCE: u8 = 15;

    pub fn new() -> BuildArea {
        return BuildArea {
            players: Vec::with_capacity(250),
            npcs: HashSet::with_capacity(250),
            appearances: HashMap::with_capacity(250),
            force_view_distance: false,
            view_distance: 15,
            last_resize: 0,
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn rebuild_npcs(&mut self) {
        // optimization to avoid sending 3 bits * observed npcs when everything has to be removed anyways
        self.npcs.clear();
    }

    #[inline(always)]
    pub fn rebuild_players(&mut self, players: &HashMap<i32, Player>, grid: &HashMap<u32, HashSet<i32>>, pid: i32, x: u16, y: u8, z: u16) {
        // optimization to avoid sending 3 bits * observed players when everything has to be removed anyways
        self.players.clear();
        self.last_resize = 0;
        self.view_distance = BuildArea::PREFERRED_VIEW_DISTANCE;
        // pre calc if we can go ahead and shorten view distance
        let mut count: u8 = 0;
        let mut decrement: bool = false;
        for _ in self.get_nearby_players(players, grid, pid, x, y, z) {
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

    #[inline(always)]
    pub fn has_appearance(&self, pid: i32, tick: u32) -> bool {
        return match self.appearances.get(&pid) {
            Some(&appearance) => appearance == tick,
            None => false,
        }
    }

    #[inline(always)]
    pub fn save_appearance(&mut self, pid: i32, tick: u32) {
        self.appearances.insert(pid, tick);
    }

    #[inline(always)]
    pub fn get_nearby_players(
        &self,
        players2: &HashMap<i32, Player>,
        grid: &HashMap<u32, HashSet<i32>>,
        pid: i32,
        x: u16,
        y: u8,
        z: u16
    ) -> Vec<i32> {
        let radius: u8 = self.view_distance * 2;
        let min: i32 = -(radius as i32) >> 1;
        let max: i32 = (radius as i32) >> 1;
        let length: i32 = (radius as i32).pow(2);

        let mut dx: i32 = 0;
        let mut dz: i32 = 0;
        let mut ldx: i32 = 0;
        let mut ldz: i32 = -1;

        let mut nearby = Vec::with_capacity(BuildArea::PREFERRED_PLAYERS as usize);

        for _ in 1..=length {
            if self.players.len() + nearby.len() >= BuildArea::PREFERRED_PLAYERS as usize {
                break;
            }
            if min < dx && dx <= max && min < dz && dz <= max {
                if let Some(players) = grid.get(&CoordGrid::from(((x as i32) + dx) as u16, y, ((z as i32) + dz) as u16).coord) {
                    for &player in players {
                        if self.players.len() + nearby.len() >= BuildArea::PREFERRED_PLAYERS as usize {
                            break;
                        }
                        if self.filter_player(players2, player, pid, x, y, z) {
                            nearby.push(player);
                        }
                    }
                }
            }

            if dx == dz || (dx < 0 && dx == -dz) || (dx > 0 && dx == 1 - dz) {
                let tmp: i32 = ldx;
                ldx = -ldz;
                ldz = tmp;
            }

            dx += ldx;
            dz += ldz;
        }

        return nearby;
    }

    #[inline(always)]
    fn filter_player(&self, players: &HashMap<i32, Player>, player: i32, pid: i32, x: u16, y: u8, z: u16) -> bool {
        if let Some(other) = players.get(&player) {
            return !(self.players.contains(&player) || !CoordGrid::within_distance_sw(&other.coord, &CoordGrid::from(x, y, z), self.view_distance) || other.pid == -1 || other.pid == pid || other.coord.y() != y);
        }
        return false;
    }
}

impl Hash for BuildArea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.players.hash(state);
        // self.appearances.hash(state);
        self.force_view_distance.hash(state);
        self.view_distance.hash(state);
        self.last_resize.hash(state);
    }
}