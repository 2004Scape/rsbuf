#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::ptr::addr_of;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::coord::CoordGrid;
use crate::info::PlayerInfo;
use crate::player::{Chat, ExactMove, Player};
use crate::renderer::PlayerRenderer;

mod coord;
mod player;

pub mod packet;
pub mod renderer;
mod prot;
mod message;
mod info;
mod build;

static mut PLAYERS: Lazy<HashMap<i32, Player>> = Lazy::new(|| HashMap::with_capacity(2048));
static mut PLAYER_GRID: Lazy<HashMap<u32, HashSet<i32>>> = Lazy::new(|| HashMap::with_capacity(2048));
static mut PLAYER_RENDERER: Lazy<PlayerRenderer> = Lazy::new(PlayerRenderer::new);
static mut PLAYER_INFO: Lazy<PlayerInfo> = Lazy::new(PlayerInfo::new);

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
    lifecycleTick: u32,
    masks: u32,
    appearance: Vec<u8>,
    lastAppearance: i32,
    faceEntity: i32,
    faceX: i32,
    faceZ: i32,
    orientationX: i32,
    orientationZ: i32,
    damageTaken: i32,
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
    exactMoveDirection: u8,
) {
    if pid == -1 {
        return;
    }

    if let Some(player) = PLAYERS.get_mut(&pid) {
        let coord: CoordGrid = CoordGrid::from(x, y, z);
        let origin: CoordGrid = CoordGrid::from(originX, y, originZ);
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

    if let Some(mut player) = PLAYERS.get_mut(&pid) {
        return PLAYER_INFO.encode(tick, pos, &mut PLAYER_RENDERER, &PLAYERS, &PLAYER_GRID, &mut player, dx, dz, rebuild);
    }

    return vec![];
}

#[wasm_bindgen(method, js_name = addPlayer)]
pub unsafe fn add_player(pid: i32) {
    if pid == -1 {
        return;
    }
    &PLAYERS.insert(pid, Player::new(pid));
}

#[wasm_bindgen(method, js_name = removePlayer)]
pub unsafe fn remove_player(pid: i32) {
    if pid == -1 {
        return;
    }
    &PLAYER_RENDERER.removePermanent(pid);
    &PLAYERS.remove(&pid);
}

#[wasm_bindgen(method, js_name = cleanup)]
pub unsafe fn cleanup() {
    &PLAYER_GRID.clear();
    &PLAYER_RENDERER.removeTemporary();
}
