use std::collections::{HashMap, HashSet};
use crate::build::BuildArea;
use crate::coord::CoordGrid;
use crate::message::{NpcInfoFaceCoord, NpcInfoFaceEntity, PlayerInfoFaceCoord, PlayerInfoFaceEntity};
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::{NpcInfoProt, PlayerInfoProt};
use crate::renderer::{NpcRenderer, PlayerRenderer};
use crate::grid::ZoneMap;
use crate::npc::Npc;

pub struct PlayerInfo {
    pub buf: Packet,
    pub updates: Packet,
}

impl PlayerInfo {
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
        players: &[Option<Player>],
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
        tick: u32,
        players: &[Option<Player>],
        renderer: &mut PlayerRenderer,
        player: &mut Player,
        mut bytes: usize
    ) -> usize {
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
    fn write_new_players(
        &mut self,
        map: &mut ZoneMap,
        players: &[Option<Player>],
        renderer: &mut PlayerRenderer,
        grid: &HashMap<u32, HashSet<i32>>,
        player: &mut Player,
        mut bytes: usize
    ) {
        for pid in player.build.get_nearby_players(players, grid, map, player.pid, player.coord.x(), player.coord.y(), player.coord.z()) {
            if player.build.players.contains(&pid) {
                continue;
            }
            if player.build.players.len() >= BuildArea::PREFERRED_PLAYERS as usize {
                return;
            }
            if let Some(other) = unsafe { &*players.as_ptr().add(pid as usize) } {
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
        player.build.players.push(other.pid);
    }

    #[inline]
    fn remove(
        &mut self,
        player: &mut Player,
        other: i32
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        player.build.players.retain(|&pid| pid != other);
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
        let myself: bool = player.pid == other.pid;
        let mut masks: u32 = other.masks;
        if myself {
            masks &= !(PlayerInfoProt::Chat as u32);
        }
        self.write_blocks(renderer, player, other, other.pid, masks, myself);
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
    fn write_blocks(
        &mut self,
        renderer: &mut PlayerRenderer,
        player: &Player,
        other: &Player,
        pid: i32,
        mut masks: u32,
        myself: bool
    ) {
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
        if masks & PlayerInfoProt::Say as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::Say);
        }
        if masks & PlayerInfoProt::Damage as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::Damage);
        }
        if masks & PlayerInfoProt::FaceCoord as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::FaceCoord);
        }
        if !myself && masks & PlayerInfoProt::Chat as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::Chat);
        }
        if masks & PlayerInfoProt::SpotAnim as u32 != 0 {
            renderer.write(&mut self.updates, pid, PlayerInfoProt::SpotAnim);
        }
        if masks & PlayerInfoProt::ExactMove as u32 != 0 {
            if let Some(exactmove) = &other.exact_move {
                let mut x = player.origin.x() as i32;
                x = ((x >> 3) - 6) << 3;
                let mut z = player.origin.z() as i32;
                z = ((z >> 3) - 6) << 3;
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
    fn fits(&self, bytes: usize, bits_to_add: usize, bytes_to_add: usize) -> bool {
        // 7 aligns to the next byte
        return ((self.buf.bit_pos + bits_to_add + 7) >> 3) + bytes + bytes_to_add <= 4997;
    }
}

pub struct NpcInfo {
    pub buf: Packet,
    pub updates: Packet,
}

impl NpcInfo {
    pub fn new() -> NpcInfo {
        return NpcInfo {
            buf: Packet::new(5000),
            updates: Packet::new(5000),
        }
    }

    #[inline]
    pub fn encode(
        &mut self,
        tick: u32,
        pos: usize,
        renderer: &mut NpcRenderer,
        npcs: &[Option<Npc>],
        map: &mut ZoneMap,
        player: &mut Player,
        dx: i32,
        dz: i32,
        rebuild: bool,
    ) -> Vec<u8> {
        let build: &mut BuildArea = &mut player.build;

        if rebuild || dx > BuildArea::PREFERRED_VIEW_DISTANCE as i32 || dz > BuildArea::PREFERRED_VIEW_DISTANCE as i32 {
            build.rebuild_npcs();
        }

        self.buf.pos = 0;
        self.buf.bit_pos = 0;
        self.updates.pos = 0;
        self.updates.bit_pos = 0;

        self.buf.bits();
        let bytes: usize = self.write_npcs(tick, npcs, renderer, player, pos);
        self.write_new_npcs(tick, map, npcs, renderer, player, bytes);
        if self.updates.pos > 0 {
            self.buf.pbit(13, 8191);
            self.buf.bytes();
            self.buf.pdata(&self.updates.data, 0, self.updates.pos);
        } else {
            self.buf.bytes();
        }
        return unsafe { self.buf.data.get_unchecked(0..self.buf.pos).to_vec() };
    }

    #[inline]
    fn write_npcs(
        &mut self,
        tick: u32,
        npcs: &[Option<Npc>],
        renderer: &mut NpcRenderer,
        player: &mut Player,
        mut bytes: usize
    ) -> usize {
        let len: usize = player.build.npcs.len();
        self.buf.pbit(8, len as i32);
        let mut index: usize = 0;
        while index < len {
            if index >= player.build.npcs.len() {
                break;
            }
            if let nid = unsafe { *player.build.npcs.as_ptr().add(index) } {
                if let Some(other) = unsafe { &*npcs.as_ptr().add(nid as usize) } {
                    if other.nid == -1 || other.tele || other.coord.y() != player.coord.y() || !CoordGrid::within_distance_sw(&player.coord, &other.coord, BuildArea::PREFERRED_VIEW_DISTANCE) || !other.check_life_cycle(tick) {
                        self.remove(player, nid);
                        index -= 1;
                    } else {
                        let len: usize = renderer.highdefinitions(nid);
                        if other.run_dir != -1 {
                            self.run(renderer, other, len > 0 && self.fits(bytes, 1 + 2 + 3 + 3 + 1, len));
                        } else if other.walk_dir != -1 {
                            self.walk(renderer, other, len > 0 && self.fits(bytes, 1 + 2 + 3 + 1, len));
                        } else if len > 0 && self.fits(bytes, 1 + 2, len) {
                            self.extend(renderer, other);
                        } else {
                            self.idle();
                        }
                        bytes += len;
                    }
                } else {
                    self.remove(player, nid);
                    index -= 1;
                }
            }
            index += 1;
        }
        return bytes;
    }

    #[inline]
    fn write_new_npcs(
        &mut self,
        tick: u32,
        map: &mut ZoneMap,
        npcs: &[Option<Npc>],
        renderer: &mut NpcRenderer,
        player: &mut Player,
        mut bytes: usize
    ) {
        for nid in player.build.get_nearby_npcs(tick, npcs, map, player.coord.x(), player.coord.y(), player.coord.z()) {
            if player.build.npcs.contains(&nid) {
                continue;
            }
            if player.build.npcs.len() >= BuildArea::PREFERRED_NPCS as usize {
                return;
            }
            if let Some(other) = unsafe { &*npcs.as_ptr().add(nid as usize) } {
                let len: usize = renderer.lowdefinitions(nid) + renderer.highdefinitions(nid);
                // bits to add npc + extended info size + bits to break loop (13)
                if !self.fits(bytes, 13 + 11 + 5 + 5 + 1 + 13, len) {
                    // more npcs get added next tick
                    return;
                }
                self.add(renderer, player, other, other.nid, other.ntype, other.coord.x() as i32 - player.coord.x() as i32, other.coord.z() as i32 - player.coord.z() as i32);
                bytes += len;
            }
        }
    }

    #[inline]
    fn add(
        &mut self,
        renderer: &mut NpcRenderer,
        player: &mut Player,
        other: &Npc,
        nid: i32,
        ntype: i32,
        x: i32,
        z: i32,
    ) {
        self.buf.pbit(13, nid);
        self.buf.pbit(11, ntype);
        self.buf.pbit(5, x);
        self.buf.pbit(5, z);
        self.buf.pbit(1, 1); // extend
        self.lowdefinition(renderer, other);
        player.build.npcs.push(other.nid);
    }

    #[inline]
    fn remove(
        &mut self,
        player: &mut Player,
        other: i32
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        player.build.npcs.retain(|&nid| nid != other);
    }

    #[inline]
    fn run(
        &mut self,
        renderer: &mut NpcRenderer,
        other: &Npc,
        extend: bool
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 2);
        self.buf.pbit(3, other.walk_dir as i32);
        self.buf.pbit(3, other.run_dir as i32);
        if extend {
            self.buf.pbit(1, 1);
            self.highdefinition(renderer, other);
        } else {
            self.buf.pbit(1, 0);
        }
    }

    #[inline]
    fn walk(
        &mut self,
        renderer: &mut NpcRenderer,
        other: &Npc,
        extend: bool
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 1);
        self.buf.pbit(3, other.walk_dir as i32);
        if extend {
            self.buf.pbit(1, 1);
            self.highdefinition(renderer, other);
        } else {
            self.buf.pbit(1, 0);
        }
    }

    #[inline]
    fn extend(
        &mut self,
        renderer: &mut NpcRenderer,
        other: &Npc
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 0);
        self.highdefinition(renderer, other);
    }

    #[inline]
    fn idle(&mut self) {
        self.buf.pbit(1, 0);
    }

    #[inline]
    fn highdefinition(
        &mut self,
        renderer: &mut NpcRenderer,
        other: &Npc
    ) {
        self.write_blocks(renderer, other.nid, other.masks);
    }

    #[inline]
    fn lowdefinition(
        &mut self,
        renderer: &mut NpcRenderer,
        other: &Npc
    ) {
        let nid: i32 = other.nid;
        let mut masks: u32 = other.masks;

        if other.face_entity != -1 && !renderer.has(nid, NpcInfoProt::FaceEntity) {
            renderer.cache(
                nid,
                &NpcInfoFaceEntity::new(other.face_entity),
                NpcInfoProt::FaceEntity,
            );
            masks |= NpcInfoProt::FaceEntity as u32;
        }

        if !renderer.has(nid, NpcInfoProt::FaceCoord) {
            if other.face_x != -1 {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(other.face_x, other.face_z),
                    NpcInfoProt::FaceCoord,
                );
            } else if other.orientation_x != -1 {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(other.orientation_x, other.orientation_z),
                    NpcInfoProt::FaceCoord,
                );
            } else {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(CoordGrid::fine(other.coord.x(), 1), CoordGrid::fine(other.coord.z(), 1)),
                    NpcInfoProt::FaceCoord,
                );
            }
        }

        masks |= NpcInfoProt::FaceCoord as u32;

        self.write_blocks(renderer, nid, masks);
    }

    #[inline]
    fn write_blocks(
        &mut self,
        renderer: &mut NpcRenderer,
        nid: i32,
        mut masks: u32,
    ) {
        self.updates.p1((masks & 0xff) as i32);
        // ----
        if masks & NpcInfoProt::Anim as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::Anim);
        }
        if masks & NpcInfoProt::FaceEntity as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::FaceEntity);
        }
        if masks & NpcInfoProt::Say as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::Say);
        }
        if masks & NpcInfoProt::Damage as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::Damage);
        }
        if masks & NpcInfoProt::ChangeType as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::ChangeType);
        }
        if masks & NpcInfoProt::SpotAnim as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::SpotAnim);
        }
        if masks & NpcInfoProt::FaceCoord as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::FaceCoord);
        }
    }

    #[inline]
    fn fits(&self, bytes: usize, bits_to_add: usize, bytes_to_add: usize) -> bool {
        // 7 aligns to the next byte
        return ((self.buf.bit_pos + bits_to_add + 7) >> 3) + bytes + bytes_to_add <= 4997;
    }
}