use crate::build::BuildArea;
use crate::coord::CoordGrid;
use crate::grid::ZoneMap;
use crate::message::{PlayerInfoFaceCoord, PlayerInfoFaceEntity};
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::PlayerInfoProt;
use crate::renderer::PlayerRenderer;
use crate::visibility::Visibility;
use std::collections::HashMap;

pub struct PlayerInfo {
    buf: Packet,
    updates: Packet,
}

impl PlayerInfo {
    const BITS_ADD: usize = 11 + 5 + 5 + 1 + 1;
    const BITS_RUN: usize = 1 + 2 + 3 + 3 + 1;
    const BITS_WALK: usize = 1 + 2 + 3 + 1;
    const BITS_EXTEND: usize = 1 + 2;

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
        pos: usize,
        renderer: &mut PlayerRenderer,
        players: &[Option<Player>],
        map: &mut ZoneMap,
        grid: &HashMap<u32, Vec<i32>>,
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
        let bytes2: usize = self.write_players(players, renderer, player, bytes1 + pos);
        self.write_new_players(map, players, renderer, grid, player, bytes2);
        if self.updates.pos > 0 {
            self.buf.pbit(11, 2047);
            self.buf.bytes();
            self.buf.pdata(&self.updates.data, 0, self.updates.pos);
        } else {
            self.buf.bytes();
        }
        return unsafe { self.buf.data.get_unchecked(0..self.buf.pos).to_vec() };
    }

    #[inline]
    fn write_local_player(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player
    ) -> usize {
        let len: usize = renderer.highdefinitions(player.pid);
        if player.tele {
            self.teleport(
                renderer,
                player,
                player,
                player.coord.x() as i32 - (((player.origin.x() as i32 >> 3) - 6) << 3),
                player.coord.y() as i32,
                player.coord.z() as i32 - (((player.origin.z() as i32 >> 3) - 6) << 3),
                player.jump,
                len > 0
            );
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
    fn write_players(
        &mut self,
        players: &[Option<Player>],
        renderer: &mut PlayerRenderer,
        player: &mut Player,
        mut bytes: usize
    ) -> usize {
        self.buf.pbit(8, player.build.players.len() as i32);
        for pid in player.build.players.iter() {
            match unsafe { &*players.as_ptr().add(pid as usize) } {
                Some(other) => {
                    if other.pid == -1 || other.tele || other.coord.y() != player.coord.y() || !CoordGrid::within_distance_sw(&player.coord, &other.coord, player.build.view_distance) || !other.active || other.visibility == Visibility::HARD {
                        self.remove(player, pid);
                    } else {
                        let len: usize = renderer.highdefinitions(pid);
                        if other.run_dir != -1 {
                            self.run(renderer, player, other, len > 0 && self.fits(bytes + 2, PlayerInfo::BITS_RUN, len));
                        } else if other.walk_dir != -1 {
                            self.walk(renderer, player, other, len > 0 && self.fits(bytes + 2, PlayerInfo::BITS_WALK, len));
                        } else if len > 0 && self.fits(bytes + 2, PlayerInfo::BITS_EXTEND, len) {
                            self.extend(renderer, player, other);
                        } else {
                            self.idle();
                        }
                        bytes += len + 2;
                    }
                }
                _ => self.remove(player, pid),
            }
        }
        return bytes;
    }

    #[inline]
    fn write_new_players(
        &mut self,
        map: &mut ZoneMap,
        players: &[Option<Player>],
        renderer: &mut PlayerRenderer,
        grid: &HashMap<u32, Vec<i32>>,
        player: &mut Player,
        mut bytes: usize
    ) {
        for pid in player.build.get_nearby_players(players, grid, map, player.pid, player.coord.x(), player.coord.y(), player.coord.z()) {
            if player.build.players.contains(pid) {
                continue;
            }
            if player.build.players.len() >= BuildArea::PREFERRED_PLAYERS as usize {
                return;
            }
            if let Some(other) = unsafe { &*players.as_ptr().add(pid as usize) } {
                if other.visibility != Visibility::HARD {
                    let len: usize = renderer.lowdefinitions(pid) + renderer.highdefinitions(pid);
                    // bits to add player + extended info size + bits to break loop (11)
                    if !self.fits(bytes + 2, PlayerInfo::BITS_ADD, len) {
                        // more players get added next tick
                        return;
                    }
                    self.add(renderer, player, other, other.pid, other.coord.x() as i32 - player.coord.x() as i32, other.coord.z() as i32 - player.coord.z() as i32, other.jump);
                    bytes += len + 2;
                }
            }
        }
    }

    #[inline]
    fn add(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &mut Player,
        other: &Player,
        pid: i32,
        x: i32,
        z: i32,
        jump: bool
    ) {
        self.buf.pbit(11, pid);
        self.buf.pbit(5, x);
        self.buf.pbit(5, z);
        self.buf.pbit(1, if jump { 1 } else { 0 });
        self.buf.pbit(1, 1); // extend
        self.lowdefinition(renderer, player, other);
        player.build.players.insert(other.pid);
    }

    #[inline]
    fn remove(
        &mut self,
        player: &mut Player,
        other: i32
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        player.build.players.remove(other);
    }

    #[inline]
    fn teleport(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player,
        x: i32,
        y: i32,
        z: i32,
        jump: bool,
        extend: bool
    ) {
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
    fn run(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player,
        extend: bool
    ) {
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
    fn walk(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player,
        extend: bool
    ) {
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
    fn extend(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 0);
        self.highdefinition(renderer, player, other);
    }

    #[inline]
    fn idle(&mut self) {
        self.buf.pbit(1, 0);
    }

    #[inline]
    fn highdefinition(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player
    ) {
        let mut masks: u32 = other.masks;
        if player.pid == other.pid {
            masks &= !(PlayerInfoProt::CHAT as u32);
        }
        self.write_blocks(renderer, player, other, masks);
    }

    #[inline]
    fn lowdefinition(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &mut Player,
        other: &Player
    ) {
        let pid: i32 = other.pid;
        let mut masks: u32 = other.masks;

        if other.last_appearance != -1 && !player.build.has_appearance(pid, other.last_appearance as u32) {
            player.build.save_appearance(pid, other.last_appearance as u32);
            masks |= PlayerInfoProt::APPEARANCE as u32;
        } else {
            masks &= !(PlayerInfoProt::APPEARANCE as u32);
        }

        if other.face_entity != -1 && !renderer.has(pid, PlayerInfoProt::FACE_ENTITY) {
            renderer.cache(
                pid,
                &PlayerInfoFaceEntity::new(other.face_entity),
                PlayerInfoProt::FACE_ENTITY,
            );
            masks |= PlayerInfoProt::FACE_ENTITY as u32;
        }

        if !renderer.has(pid, PlayerInfoProt::FACE_COORD) {
            if other.face_x != -1 {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(other.face_x, other.face_z),
                    PlayerInfoProt::FACE_COORD,
                );
            } else if other.orientation_x != -1 {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(other.orientation_x, other.orientation_z),
                    PlayerInfoProt::FACE_COORD,
                );
            } else {
                renderer.cache(
                    pid,
                    &PlayerInfoFaceCoord::new(CoordGrid::fine(other.coord.x(), 1), CoordGrid::fine(other.coord.z(), 1)),
                    PlayerInfoProt::FACE_COORD,
                );
            }
        }

        masks |= PlayerInfoProt::FACE_COORD as u32;

        self.write_blocks(renderer, player, other, masks);
    }

    #[inline]
    fn write_blocks(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player,
        masks: u32,
    ) {
        if masks > 0xff {
            self.updates.ip2(masks as i32 | PlayerInfoProt::BIG as i32);
        } else {
            self.updates.p1(masks as i32);
        }
        // ----
        if masks & PlayerInfoProt::APPEARANCE as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::APPEARANCE);
        }
        if masks & PlayerInfoProt::ANIM as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::ANIM);
        }
        if masks & PlayerInfoProt::FACE_ENTITY as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::FACE_ENTITY);
        }
        if masks & PlayerInfoProt::SAY as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::SAY);
        }
        if masks & PlayerInfoProt::DAMAGE as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::DAMAGE);
        }
        if masks & PlayerInfoProt::FACE_COORD as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::FACE_COORD);
        }
        if masks & PlayerInfoProt::CHAT as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::CHAT);
        }
        if masks & PlayerInfoProt::SPOT_ANIM as u32 != 0 {
            renderer.write(&mut self.updates, other.pid, PlayerInfoProt::SPOT_ANIM);
        }
        if masks & PlayerInfoProt::EXACT_MOVE as u32 != 0 {
            if let Some(exactmove) = &other.exact_move {
                let x: i32 = (((player.origin.x() as i32) >> 3) - 6) << 3;
                let z: i32 = (((player.origin.z() as i32) >> 3) - 6) << 3;
                renderer.writeExactmove(
                    &mut self.updates,
                    exactmove.start_x - x,
                    exactmove.start_z - z,
                    exactmove.end_x - x,
                    exactmove.end_z - z,
                    exactmove.begin,
                    exactmove.finish,
                    exactmove.dir,
                )
            }
        }
    }

    #[inline]
    const fn fits(&self, bytes: usize, bits_to_add: usize, bytes_to_add: usize) -> bool {
        // 7 aligns to the next byte
        return ((self.buf.bit_pos + bits_to_add + 7) >> 3) + bytes + bytes_to_add <= 4997;
    }
}