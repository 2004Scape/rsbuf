#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::coord::CoordGrid;
use crate::info::{NpcInfo, PlayerInfo};
use crate::player::{Chat, ExactMove, Player};
use crate::renderer::{NpcRenderer, PlayerRenderer};
use crate::grid::ZoneMap;
use crate::npc::Npc;

mod coord;
mod player;

pub mod packet;
pub mod renderer;
mod prot;
mod message;
mod info;
mod build;
mod grid;
mod npc;

static mut PLAYERS: Lazy<Vec<Option<Player>>> = Lazy::new(|| vec![None; 2048]);
static mut PLAYER_GRID: Lazy<HashMap<u32, HashSet<i32>>> = Lazy::new(|| HashMap::with_capacity(2048));
static mut PLAYER_RENDERER: Lazy<PlayerRenderer> = Lazy::new(PlayerRenderer::new);
static mut PLAYER_INFO: Lazy<PlayerInfo> = Lazy::new(PlayerInfo::new);

static mut NPCS: Lazy<Vec<Option<Npc>>> = Lazy::new(|| vec![None; 8192]);
static mut NPC_RENDERER: Lazy<NpcRenderer> = Lazy::new(NpcRenderer::new);
static mut NPC_INFO: Lazy<NpcInfo> = Lazy::new(NpcInfo::new);

static mut ZONE_MAP: Lazy<ZoneMap> = Lazy::new(ZoneMap::new);

#[wasm_bindgen(method, js_name = computePlayer)]
pub unsafe fn compute_player(
    x: u16,
    y: u8,
    z: u16,
    originX: u16,
    originZ: u16,
    pid: i32,
    tele: bool,
    jump: bool,
    runDir: i8,
    walkDir: i8,
    visibility: u8,
    lifecycle: u8,
    lifecycleTick: i32,
    masks: u32,
    appearance: Vec<u8>,
    lastAppearance: i32,
    faceEntity: i32,
    faceX: i32,
    faceZ: i32,
    orientationX: i32,
    orientationZ: i32,
    damageTaken: i32,
    damageType: i32,
    currentHitpoints: i32,
    baseHitpoints: i32,
    animId: i32,
    animDelay: i32,
    say: Option<String>,
    message: Option<Vec<u8>>,
    color: u8,
    effect: u8,
    ignored: u8,
    graphicId: i32,
    graphicHeight: i32,
    graphicDelay: i32,
    exactStartX: i32,
    exactStartZ: i32,
    exactEndX: i32,
    exactEndZ: i32,
    exactMoveStart: i32,
    exactMoveEnd: i32,
    exactMoveDirection: i32,
) {
    if pid == -1 {
        return;
    }

    if let Some(Some(player)) = PLAYERS.get_mut(pid as usize) {
        let origin: CoordGrid = CoordGrid::from(originX, y, originZ);
        let coord: CoordGrid = CoordGrid::from(x, y, z);
        let exact_move: Option<ExactMove> = match exactStartX {
            -1 => None,
            _ => Some(
                ExactMove::new(
                    exactStartX,
                    exactStartZ,
                    exactEndX,
                    exactEndZ,
                    exactMoveStart,
                    exactMoveEnd,
                    exactMoveDirection,
                )
            )
        };
        let chat: Option<Chat> = match message {
            None => None,
            Some(bytes) => Some(
                Chat::new(
                    bytes,
                    color,
                    effect,
                    ignored,
                )
            )
        };

        if coord.coord != player.coord.coord {
            &ZONE_MAP.zone(player.coord.x(), player.coord.y(), player.coord.z()).remove_player(pid);
            &ZONE_MAP.zone(coord.x(), coord.y(), coord.z()).add_player(pid);
        }

        player.coord = coord;
        player.origin = origin;
        player.tele = tele;
        player.jump = jump;
        player.run_dir = runDir;
        player.walk_dir = walkDir;
        player.visibility = visibility;
        player.lifecycle = lifecycle;
        player.lifecycle_tick = lifecycleTick;
        player.masks = masks;
        player.appearance = appearance;
        player.last_appearance = lastAppearance;
        player.face_entity = faceEntity;
        player.face_x = faceX;
        player.face_z = faceZ;
        player.orientation_x = orientationX;
        player.orientation_z = orientationZ;
        player.damage_taken = damageTaken;
        player.damage_type = damageType;
        player.current_hitpoints = currentHitpoints;
        player.base_hitpoints = baseHitpoints;
        player.anim_id = animId;
        player.anim_delay = animDelay;
        player.say = say;
        player.chat = chat;
        player.graphic_id = graphicId;
        player.graphic_height = graphicHeight;
        player.graphic_delay = graphicDelay;
        player.exact_move = exact_move;

        &PLAYER_RENDERER.compute_info(&player);
        &PLAYER_GRID.entry(player.coord.coord).or_insert_with(HashSet::new).insert(pid);
    }
}

#[wasm_bindgen(method, js_name = playerInfo)]
pub unsafe fn player_info(tick: u32, pos: usize, pid: i32, dx: i32, dz: i32, rebuild: bool) -> Vec<u8> {
    if pid == -1 {
        return vec![];
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return PLAYER_INFO.encode(
            tick,
            pos,
            &mut PLAYER_RENDERER,
            &PLAYERS,
            &mut ZONE_MAP,
            &PLAYER_GRID,
            player,
            dx,
            dz,
            rebuild,
        );
    }

    return vec![];
}

#[wasm_bindgen(method, js_name = addPlayer)]
pub unsafe fn add_player(pid: i32) {
    if pid == -1 {
        return;
    }
    *PLAYERS.as_mut_ptr().add(pid as usize) = Some(Player::new(pid));
}

#[wasm_bindgen(method, js_name = removePlayer)]
pub unsafe fn remove_player(pid: i32) {
    if pid == -1 {
        return;
    }
    &PLAYER_RENDERER.removePermanent(pid);
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        player.build.cleanup();
    }
    *PLAYERS.as_mut_ptr().add(pid as usize) = None;
}

#[wasm_bindgen(method, js_name = hasPlayer)]
pub unsafe fn has_player(pid: i32, other: i32) -> bool {
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        return player.build.players.contains(&other);
    }
    return false;
}

#[wasm_bindgen(method, js_name = computeNpc)]
pub unsafe fn compute_npc(
    x: u16,
    y: u8,
    z: u16,
    nid: i32,
    ntype: i32,
    tele: bool,
    runDir: i8,
    walkDir: i8,
    lifecycle: u8,
    lifecycleTick: i32,
    masks: u32,
    faceEntity: i32,
    faceX: i32,
    faceZ: i32,
    orientationX: i32,
    orientationZ: i32,
    damageTaken: i32,
    damageType: i32,
    currentHitpoints: i32,
    baseHitpoints: i32,
    animId: i32,
    animDelay: i32,
    say: Option<String>,
    graphicId: i32,
    graphicHeight: i32,
    graphicDelay: i32,
) {
    if nid == -1 || ntype == -1 {
        return;
    }

    if let Some(Some(npc)) = NPCS.get_mut(nid as usize) {
        let coord: CoordGrid = CoordGrid::from(x, y, z);

        if coord.coord != npc.coord.coord {
            &ZONE_MAP.zone(npc.coord.x(), npc.coord.y(), npc.coord.z()).remove_npc(nid);
            &ZONE_MAP.zone(coord.x(), coord.y(), coord.z()).add_npc(nid);
        }

        npc.ntype = ntype;
        npc.coord = coord;
        npc.tele = tele;
        npc.run_dir = runDir;
        npc.walk_dir = walkDir;
        npc.lifecycle = lifecycle;
        npc.lifecycle_tick = lifecycleTick;
        npc.masks = masks;
        npc.face_entity = faceEntity;
        npc.face_x = faceX;
        npc.face_z = faceZ;
        npc.orientation_x = orientationX;
        npc.orientation_z = orientationZ;
        npc.damage_taken = damageTaken;
        npc.damage_type = damageType;
        npc.current_hitpoints = currentHitpoints;
        npc.base_hitpoints = baseHitpoints;
        npc.anim_id = animId;
        npc.anim_delay = animDelay;
        npc.say = say;
        npc.graphic_id = graphicId;
        npc.graphic_height = graphicHeight;
        npc.graphic_delay = graphicDelay;

        &NPC_RENDERER.compute_info(&npc);
    }
}

#[wasm_bindgen(method, js_name = npcInfo)]
pub unsafe fn npc_info(tick: u32, pos: usize, pid: i32, dx: i32, dz: i32, rebuild: bool) -> Vec<u8> {
    if pid == -1 {
        return vec![];
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return NPC_INFO.encode(tick, pos, &mut NPC_RENDERER, &NPCS, &mut ZONE_MAP, player, dx, dz, rebuild);
    }

    return vec![];
}

#[wasm_bindgen(method, js_name = addNpc)]
pub unsafe fn add_npc(nid: i32, ntype: i32) {
    if nid == -1 || ntype == -1 {
        return;
    }
    *NPCS.as_mut_ptr().add(nid as usize) = Some(Npc::new(nid, ntype));
}

#[wasm_bindgen(method, js_name = removeNpc)]
pub unsafe fn remove_npc(nid: i32) {
    if nid == -1 {
        return;
    }
    &NPC_RENDERER.removePermanent(nid);
    *NPCS.as_mut_ptr().add(nid as usize) = None;
}

#[wasm_bindgen(method, js_name = hasNpc)]
pub unsafe fn has_npc(pid: i32, nid: i32) -> bool {
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        return player.build.npcs.contains(&nid);
    }
    return false;
}

#[wasm_bindgen(method, js_name = cleanup)]
pub unsafe fn cleanup() {
    &PLAYER_GRID.clear();
    &PLAYER_RENDERER.removeTemporary();
    &NPC_RENDERER.removeTemporary();
    for player in PLAYERS.iter_mut() {
        if let Some(player) = player {
            player.cleanup();
        }
    }
    for npc in NPCS.iter_mut() {
        if let Some(npc) = npc {
            npc.cleanup();
        }
    }
}
