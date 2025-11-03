use crate::build::BuildArea;
use crate::coord::CoordGrid;
use crate::grid::ZoneMap;
use crate::message::{NpcInfoFaceCoord, NpcInfoFaceEntity};
use crate::npc::Npc;
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::NpcInfoProt;
use crate::renderer::NpcRenderer;

pub struct NpcInfo {
    buf: Packet,
    updates: Packet,
}

impl NpcInfo {
    const BITS_ADD: usize = 13 + 11 + 5 + 5 + 1;
    const BITS_RUN: usize = 1 + 2 + 3 + 3 + 1;
    const BITS_WALK: usize = 1 + 2 + 3 + 1;
    const BITS_EXTEND: usize = 1 + 2;

    #[inline]
    pub fn new() -> NpcInfo {
        return NpcInfo {
            buf: Packet::new(5000),
            updates: Packet::new(5000),
        }
    }

    #[inline]
    pub fn encode(
        &mut self,
        renderer: &mut NpcRenderer,
        npcs: &mut [Option<Npc>],
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
        let bytes: usize = self.write_npcs(npcs, renderer, player, 0);
        self.write_new_npcs(map, npcs, renderer, player, bytes);
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
        npcs: &mut [Option<Npc>],
        renderer: &mut NpcRenderer,
        player: &mut Player,
        mut bytes: usize
    ) -> usize {
        self.buf.pbit(8, player.build.npcs.len() as i32);
        for nid in player.build.npcs.iter() {
            match unsafe { &mut *npcs.as_mut_ptr().add(nid as usize) } {
                Some(other) => {
                    if other.nid == -1 || other.tele || other.coord.y() != player.coord.y() || !CoordGrid::within_distance_sw(&player.coord, &other.coord, BuildArea::PREFERRED_VIEW_DISTANCE) || !other.active {
                        self.remove(player, nid);
                        other.observers = (other.observers - 1).max(0);
                    } else {
                        let len: usize = renderer.highdefinitions(nid);
                        if other.run_dir != -1 {
                            self.run(renderer, other, len > 0 && self.fits(bytes + 1, NpcInfo::BITS_RUN, len));
                        } else if other.walk_dir != -1 {
                            self.walk(renderer, other, len > 0 && self.fits(bytes + 1, NpcInfo::BITS_WALK, len));
                        } else if len > 0 && self.fits(bytes + 1, NpcInfo::BITS_EXTEND, len) {
                            self.extend(renderer, other);
                        } else {
                            self.idle();
                        }
                        bytes += len + 1;
                    }
                }
                _ => self.remove(player, nid),
            }
        }
        return bytes;
    }

    #[inline]
    fn write_new_npcs(
        &mut self,
        map: &mut ZoneMap,
        npcs: &mut [Option<Npc>],
        renderer: &mut NpcRenderer,
        player: &mut Player,
        mut bytes: usize
    ) {
        for nid in player.build.get_nearby_npcs(npcs, map, player.coord.x(), player.coord.y(), player.coord.z()) {
            if player.build.npcs.contains(nid) {
                continue;
            }
            if player.build.npcs.len() >= BuildArea::PREFERRED_NPCS as usize {
                return;
            }
            if let Some(other) = unsafe { &mut *npcs.as_mut_ptr().add(nid as usize) } {
                let len: usize = renderer.lowdefinitions(nid) + renderer.highdefinitions(nid);
                // bits to add npc + extended info size + bits to break loop (13)
                if !self.fits(bytes + 1, NpcInfo::BITS_ADD, len) {
                    // more npcs get added next tick
                    return;
                }
                self.add(renderer, player, other, other.nid, other.ntype, other.coord.x() as i32 - player.coord.x() as i32, other.coord.z() as i32 - player.coord.z() as i32);
                other.observers = other.observers + 1;
                bytes += len + 1;
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
        player.build.npcs.insert(other.nid);
    }

    #[inline]
    fn remove(
        &mut self,
        player: &mut Player,
        other: i32
    ) {
        self.buf.pbit(1, 1);
        self.buf.pbit(2, 3);
        player.build.npcs.remove(other);
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

        if other.face_entity != -1 && !renderer.has(nid, NpcInfoProt::FACE_ENTITY) {
            renderer.cache(
                nid,
                &NpcInfoFaceEntity::new(other.face_entity),
                NpcInfoProt::FACE_ENTITY,
            );
            masks |= NpcInfoProt::FACE_ENTITY as u32;
        }

        if !renderer.has(nid, NpcInfoProt::FACE_COORD) {
            if other.face_x != -1 {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(other.face_x, other.face_z),
                    NpcInfoProt::FACE_COORD,
                );
            } else if other.orientation_x != -1 {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(other.orientation_x, other.orientation_z),
                    NpcInfoProt::FACE_COORD,
                );
            } else {
                renderer.cache(
                    nid,
                    &NpcInfoFaceCoord::new(CoordGrid::fine(other.coord.x(), 1), CoordGrid::fine(other.coord.z(), 1)),
                    NpcInfoProt::FACE_COORD,
                );
            }
        }

        masks |= NpcInfoProt::FACE_COORD as u32;

        self.write_blocks(renderer, nid, masks);
    }

    #[inline]
    fn write_blocks(
        &mut self,
        renderer: &mut NpcRenderer,
        nid: i32,
        masks: u32,
    ) {
        self.updates.p1((masks & 0xff) as i32);
        // ----
        // an optimization *could* be made where all of these are just 1 block of bytes...
        // the same could NOT be done for players bcuz of how exact_move works...
        if masks & NpcInfoProt::ANIM as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::ANIM);
        }
        if masks & NpcInfoProt::FACE_ENTITY as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::FACE_ENTITY);
        }
        if masks & NpcInfoProt::SAY as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::SAY);
        }
        if masks & NpcInfoProt::DAMAGE as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::DAMAGE);
        }
        if masks & NpcInfoProt::CHANGE_TYPE as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::CHANGE_TYPE);
        }
        if masks & NpcInfoProt::SPOT_ANIM as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::SPOT_ANIM);
        }
        if masks & NpcInfoProt::FACE_COORD as u32 != 0 {
            renderer.write(&mut self.updates, nid, NpcInfoProt::FACE_COORD);
        }
    }

    #[inline]
    const fn fits(&self, bytes: usize, bits_to_add: usize, bytes_to_add: usize) -> bool {
        // 7 aligns to the next byte
        return ((self.buf.bit_pos + bits_to_add + 7) >> 3) + bytes + bytes_to_add <= 4997;
    }
}