use std::collections::{HashMap, HashSet};
use crate::build::BuildArea;
use crate::coord::CoordGrid;
use crate::message::{PlayerInfoFaceCoord, PlayerInfoFaceEntity};
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::PlayerInfoProt;
use crate::renderer::PlayerRenderer;
use crate::zone::ZoneMap;

pub struct PlayerInfo {
    pub buf: Packet,
    pub updates: Packet,
}

impl PlayerInfo {
    #[inline]
    pub fn new() -> PlayerInfo {
        return PlayerInfo {
            buf: Packet::new(5000),
            updates: Packet::new(5000),
        }
    }

    #[inline]
    pub fn encode(
        &mut self,
        tick: u32,
        pos: usize,
        renderer: &mut PlayerRenderer,
        players: &Vec<Option<Player>>,
        map: &mut ZoneMap,
        grid: &HashMap<u32, HashSet<i32>>,
        player: &mut Player,
        dx: i32,
        dz: i32,
        rebuild: bool,
    ) -> Vec<u8> {
        let build: &mut BuildArea = &mut player.build;

        if rebuild || dx > build.view_distance as i32 || dz > build.view_distance as i32 {
            build.rebuild_players(players, grid, player.pid, player.coord.x(), player.coord.y(), player.coord.z());
        } else {
            build.resize();
        }

        self.buf.pos = 0;
        self.buf.bit_pos = 0;
        self.updates.pos = 0;
        self.updates.bit_pos = 0;

        self.buf.bits();
        let bytes1: usize = self.write_local_player(renderer, player);
        let bytes2: usize = self.write_players(tick, players, renderer, player, bytes1 + pos);
        self.write_new_players(map, players, renderer, grid, player, bytes2);
        if self.updates.pos > 0 {
            self.buf.pbit(11, 2047);
            self.buf.bytes();
            self.buf.pdata(&self.updates.data, 0, self.updates.pos);
        } else {
            self.buf.bytes();
        }
        return self.buf.data[0..self.buf.pos].to_vec();
    }

    #[inline]
    fn write_local_player(&mut self, renderer: &mut PlayerRenderer, player: &Player) -> usize {
        let len: usize = renderer.highdefinitions(player.pid);
        if player.tele {
            let x: i32 = player.coord.x() as i32;
            let z: i32 = player.coord.z() as i32;
            self.teleport(renderer, player, player, x - (((x >> 3) - 6) << 3), player.coord.y() as i32, z - (((z >> 3) - 6) << 3), player.jump, len > 0);
        } else if player.run_dir != -1 {
            self.run(renderer, player, player, len > 0);
        } else if player.walk_dir != -1 {
            self.walk(renderer, player, player, len > 0);
        } else if len > 0 {
            self.extend(renderer, player, player);
        } else {
            self.idle();
        }
        return len;
    }

    #[inline]
    fn write_players(&mut self, tick: u32, players: &Vec<Option<Player>>, renderer: &mut PlayerRenderer, player: &mut Player, mut bytes: usize) -> usize {
        let len: usize = player.build.players.len();
        self.buf.pbit(8, len as i32);
        let mut index: usize = 0;
        while index < len {
            if index >= player.build.players.len() {
                break;
            }
            if let pid = unsafe { *player.build.players.as_ptr().add(index) } {
                if let Some(other) = unsafe { &*players.as_ptr().add(pid as usize) } {
                    if other.pid == -1 || other.tele || other.coord.y() != player.coord.y() || !CoordGrid::within_distance_sw(&player.coord, &other.coord, player.build.view_distance) || !other.check_life_cycle(tick) || other.visibility == 2 {
                        self.remove(player, pid);
                        index -= 1;
                    } else {
                        let len: usize = renderer.highdefinitions(pid);
                        if other.run_dir != -1 {
                            self.run(renderer, player, other, len > 0 && self.fits(bytes, 1 + 2 + 3 + 3 + 1, len));
                        } else if other.walk_dir != -1 {
                            self.walk(renderer, player, other, len > 0 && self.fits(bytes, 1 + 2 + 3 + 1, len));
                        } else if len > 0 && self.fits(bytes, 1 + 2, len) {
                            self.extend(renderer, player, other);
                        } else {
                            self.idle();
                        }
                        bytes += len;
                    }
                } else {
                    self.remove(player, pid);
                    index -= 1;
                }
            }
            index += 1;
        }
        return bytes;
    }

    #[inline]
    fn write_new_players(&mut self, map: &mut ZoneMap, players: &Vec<Option<Player>>, renderer: &mut PlayerRenderer, grid: &HashMap<u32, HashSet<i32>>, player: &mut Player, mut bytes: usize) {
        for pid in player.build.get_nearby_players(players, grid, map, player.pid, player.coord.x(), player.coord.y(), player.coord.z()) {
            if player.build.players.len() >= BuildArea::PREFERRED_PLAYERS as usize {
                return;
            }
            if let Some(Some(other)) = players.get(pid as usize) {
                if other.visibility != 2 {
                    let len: usize = renderer.lowdefinitions(pid) + renderer.highdefinitions(pid);
                    // bits to add player + extended info size + bits to break loop (11)
                    if !self.fits(bytes, 11 + 5 + 5 + 1 + 1 + 11, len) {
                        // more players get added next tick
                        return;
                    }
                    self.add(renderer, player, other, other.pid, other.coord.x() as i32 - player.coord.x() as i32, other.coord.z() as i32 - player.coord.z() as i32, other.jump);
                    bytes += len;
                }
            }
        }
    }

    #[inline]
    fn add(&mut self, renderer: &mut PlayerRenderer, player: &mut Player, other: &Player, pid: i32, x: i32, z: i32, jump: bool) {
        self.buf.pbit(11, pid);
        self.buf.pbit(5, x);
        self.buf.pbit(5, z);
        self.buf.pbit(1, if jump { 1 } else { 0 });
        self.buf.pbit(1, 1); // extend
        self.lowdefinition(renderer, player, other);
        player.build.players.push(other.pid);
    }

    #[inline]
    fn remove(&mut self, player: &mut Player, other: i32) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        player.build.players.retain(|&pid| pid != other);
    }

    #[inline]
    fn teleport(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player, x: i32, y: i32, z: i32, jump: bool, extend: bool) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        self.buf.pbit(2, y);
        self.buf.pbit(7, x);
        self.buf.pbit(7, z);
        self.buf.pbit(1, if jump { 1 } else { 0 });
        if extend {
            self.buf.pbit(1, 1);
            self.highdefinition(renderer, player, other);
        } else {
            self.buf.pbit(1, 0);
        }
    }

    #[inline]
    fn run(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player, extend: bool) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 2);
        self.buf.pbit(3, other.walk_dir as i32);
        self.buf.pbit(3, other.run_dir as i32);
        if extend {
            self.buf.pbit(1, 1);
            self.highdefinition(renderer, player, other);
        } else {
            self.buf.pbit(1, 0);
        }
    }

    #[inline]
    fn walk(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player, extend: bool) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 1);
        self.buf.pbit(3, other.walk_dir as i32);
        if extend {
            self.buf.pbit(1, 1);
            self.highdefinition(renderer, player, other);
        } else {
            self.buf.pbit(1, 0);
        }
    }

    #[inline]
    fn extend(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 0);
        self.highdefinition(renderer, player, other);
    }

    #[inline]
    fn idle(&mut self) {
        self.buf.pbit(1, 0);
    }

    #[inline]
    fn highdefinition(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player) {
        let myself: bool = player.pid == other.pid;
        let mut masks: u32 = other.masks;
        if myself {
            masks &= !(PlayerInfoProt::Chat as u32);
        }
        self.write_blocks(renderer, player, other, other.pid, masks, myself);
    }

    #[inline]
    fn lowdefinition(&mut self, renderer: &mut PlayerRenderer, player: &mut Player, other: &Player) {
        let pid: i32 = other.pid;
        let mut masks: u32 = other.masks;

        if other.last_appearance != -1 && !player.build.has_appearance(pid, other.last_appearance as u32) {
            player.build.save_appearance(pid, other.last_appearance as u32);
            masks |= PlayerInfoProt::Appearance as u32;
        } else {
            masks &= !(PlayerInfoProt::Appearance as u32);
        }

        if other.face_entity != -1 && !renderer.has(pid, PlayerInfoProt::FaceEntity) {
            renderer.cache(
                pid,
                &PlayerInfoFaceEntity::new(other.face_entity),
                PlayerInfoProt::FaceEntity,
            );
            masks |= PlayerInfoProt::FaceEntity as u32;
        }

        if !renderer.has(pid, PlayerInfoProt::FaceCoord) {
            if other.face_x != -1 {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(other.face_x, other.face_z),
                    PlayerInfoProt::FaceCoord,
                );
            } else if other.orientation_x != -1 {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(other.orientation_x, other.orientation_z),
                    PlayerInfoProt::FaceCoord,
                );
            } else {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(CoordGrid::fine(other.coord.x(), 1), CoordGrid::fine(other.coord.z(), 1)),
                    PlayerInfoProt::FaceCoord,
                );
            }
        }

        masks |= PlayerInfoProt::FaceCoord as u32;

        self.write_blocks(renderer, player, other, pid, masks, false);
    }

    #[inline]
    fn write_blocks(&mut self, renderer: &mut PlayerRenderer, player: &Player, other: &Player, pid: i32, mut masks: u32, myself: bool) {
        if masks > 0xff {
            masks |= PlayerInfoProt::Big as u32;
        }
        self.updates.p1((masks & 0xff) as i32);
        if masks & PlayerInfoProt::Big as u32 != 0 {
            self.updates.p1((masks >> 8) as i32);
        }
        // ----
        if masks & PlayerInfoProt::Appearance as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::Appearance);
        }
        if masks & PlayerInfoProt::Anim as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::Anim);
        }
        if masks & PlayerInfoProt::FaceEntity as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::FaceEntity);
        }
        if masks & PlayerInfoProt::FaceCoord as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::FaceCoord);
        }
    }

    #[inline]
    fn fits(&self, bytes: usize, bits_to_add: usize, bytes_to_add: usize) -> bool {
        // 7 aligns to the next byte
        return ((self.buf.bit_pos + bits_to_add + 7) >> 3) + bytes + bytes_to_add <= 4997;
    }
}