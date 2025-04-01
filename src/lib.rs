#![allow(non_snake_case)]

use crate::coord::CoordGrid;
use crate::grid::ZoneMap;
use crate::message::{IncomingPacket, MessageDecoder};
use crate::npc::Npc;
use crate::out::cam_lookat::CamLookAt;
use crate::out::cam_moveto::CamMoveTo;
use crate::out::cam_reset::CamReset;
use crate::out::cam_shake::CamShake;
use crate::out::chat_filter_settings::ChatFilterSettings;
use crate::out::count_dialog::PCountDialog;
use crate::out::data_land::{DataLand, DataLandDone};
use crate::out::data_loc::{DataLoc, DataLocDone};
use crate::out::enable_tracking::EnableTracking;
use crate::out::finish_tracking::FinishTracking;
use crate::out::hint_arrow::HintArrow;
use crate::out::if_close::IfClose;
use crate::out::if_openchat::IfOpenChat;
use crate::out::if_openmain::IfOpenMain;
use crate::out::if_openmainside::IfOpenMainSide;
use crate::out::if_openside::IfOpenSide;
use crate::out::if_setanim::IfSetAnim;
use crate::out::if_setcolour::IfSetColour;
use crate::out::if_sethide::IfSetHide;
use crate::out::if_setmodel::IfSetModel;
use crate::out::if_setnpchead::IfSetNpcHead;
use crate::out::if_setobject::IfSetObject;
use crate::out::if_setplayerhead::IfSetPlayerHead;
use crate::out::if_setposition::IfSetPosition;
use crate::out::if_setrecol::IfSetRecol;
use crate::out::if_settab::IfSetTab;
use crate::out::if_settabactive::IfSetTabActive;
use crate::out::if_settext::IfSetText;
use crate::out::last_login_info::LastLoginInfo;
use crate::out::loc_addchange::LocAddChange;
use crate::out::loc_anim::LocAnim;
use crate::out::loc_del::LocDel;
use crate::out::loc_merge::LocMerge;
use crate::out::logout::Logout;
use crate::out::map_anim::MapAnim;
use crate::out::map_projanim::MapProjAnim;
use crate::out::message_game::MessageGame;
use crate::out::message_private::MessagePrivateOut;
use crate::out::midi_jingle::MidiJingle;
use crate::out::midi_song::MidiSong;
use crate::out::npc_info::NpcInfo;
use crate::out::obj_add::ObjAdd;
use crate::out::obj_count::ObjCount;
use crate::out::obj_del::ObjDel;
use crate::out::obj_reveal::ObjReveal;
use crate::out::rebuild_normal::RebuildNormal;
use crate::out::reset_anims::ResetAnims;
use crate::out::reset_clientvarcache::ResetClientVarCache;
use crate::out::set_multiway::SetMultiway;
use crate::out::synth_sound::SynthSound;
use crate::out::tut_flash::TutFlash;
use crate::out::tut_open::TutOpen;
use crate::out::unset_map_flag::UnsetMapFlag;
use crate::out::update_friendlist::UpdateFriendList;
use crate::out::update_ignorelist::UpdateIgnoreList;
use crate::out::update_inv_full::UpdateInvFull;
use crate::out::update_inv_partial::UpdateInvPartial;
use crate::out::update_inv_stop_transmit::UpdateInvStopTransmit;
use crate::out::update_pid::UpdatePid;
use crate::out::update_reboot_timer::UpdateRebootTimer;
use crate::out::update_runenergy::UpdateRunEnergy;
use crate::out::update_runweight::UpdateRunWeight;
use crate::out::update_stat::UpdateStat;
use crate::out::update_zone_full_follows::UpdateZoneFullFollows;
use crate::out::update_zone_partial_enclosed::UpdateZonePartialEnclosed;
use crate::out::update_zone_partial_follows::UpdateZonePartialFollows;
use crate::out::varp_large::VarpLarge;
use crate::out::varp_small::VarpSmall;
use crate::packet::Packet;
use crate::player::{Chat, ExactMove, Player};
use crate::pool::PacketPool;
use crate::prot::{ClientInternalProt, ClientProt, ServerInternalProt};
use crate::r#in::anticheat::{AnticheatCycle1, AnticheatCycle2, AnticheatCycle3, AnticheatCycle4, AnticheatCycle5, AnticheatCycle6, AnticheatOp1, AnticheatOp2, AnticheatOp3, AnticheatOp4, AnticheatOp5, AnticheatOp6, AnticheatOp7, AnticheatOp8, AnticheatOp9};
use crate::r#in::chat_setmode::ChatSetMode;
use crate::r#in::client_cheat::ClientCheat;
use crate::r#in::close_modal::CloseModal;
use crate::r#in::event::{EventCameraPosition, EventTracking};
use crate::r#in::friend::{FriendListAdd, FriendListDel};
use crate::r#in::idle_timer::IdleTimer;
use crate::r#in::if_button::IfButton;
use crate::r#in::if_playerdesign::IfPlayerDesign;
use crate::r#in::ignore::{IgnoreListAdd, IgnoreListDel};
use crate::r#in::inv_button::{InvButton, InvButtonD};
use crate::r#in::message_private::MessagePrivate;
use crate::r#in::message_public::MessagePublic;
use crate::r#in::move_click::MoveClick;
use crate::r#in::no_timeout::NoTimeout;
use crate::r#in::opheld::{OpHeld, OpHeldT, OpHeldU};
use crate::r#in::oploc::{OpLoc, OpLocT, OpLocU};
use crate::r#in::opnpc::{OpNpc, OpNpcT, OpNpcU};
use crate::r#in::opobj::{OpObj, OpObjT, OpObjU};
use crate::r#in::opplayer::{OpPlayer, OpPlayerT, OpPlayerU};
use crate::r#in::rebuild_getmaps::RebuildGetMaps;
use crate::r#in::reportabuse::ReportAbuse;
use crate::r#in::resume_countdialog::ResumePCountDialog;
use crate::r#in::resume_pausebutton::ResumePauseButton;
use crate::r#in::tutorial_clickside::TutorialClickSide;
use crate::renderer::{NpcRenderer, PlayerRenderer};
use crate::visibility::Visibility;
use crate::wordpack::WordPack;
use once_cell::sync::Lazy;
use out::player_info::PlayerInfo;
use std::collections::HashMap;
use std::ptr::{addr_of, addr_of_mut};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod packet;
pub mod renderer;
pub mod build;
pub mod wordpack;

mod coord;
mod player;
mod prot;
mod message;
mod grid;
mod npc;
mod visibility;
mod category;
mod r#in;
mod out;
mod pool;

macro_rules! read {
    ($fn_name:ident, $js_name:literal, $struct:ty, $prot:expr) => {
        #[wasm_bindgen(js_name = $js_name)]
        pub unsafe fn $fn_name(bytes: Vec<u8>) -> $struct {
            <$struct>::decode($prot, Packet::from(bytes))
        }
    };
}

// these priorities are important for cases where the content developer wants to be aware of the
// bandwidth implications their script may run into and how it impacts the player experience

// counted as part of the buffer_full command
// alternate names: LOW, CONTENT
macro_rules! buffer {
    ($fn_name:ident, $js_name:literal, $struct:ty, ($($arg_name:ident: $arg_ty:ty),*), ($($arg_val:ident),*)) => {
        #[wasm_bindgen(js_name = $js_name)]
        pub unsafe fn $fn_name(pid: i32, $($arg_name: $arg_ty),*) -> Option<Vec<u8>> {
            if pid == -1 {
                return None;
            }
            match &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
                None => None,
                Some(player) => {
                    player.buffer(&mut *POOL, &<$struct>::new($($arg_val),*));
                    None
                },
            }
        }
    };
}

// not counted as part of the buffer_full command
// alternate names: HIGH, ESSENTIAL, ENGINE
macro_rules! write {
    ($fn_name:ident, $js_name:literal, $struct:ty, ($($arg_name:ident: $arg_ty:ty),*), ($($arg_val:ident),*)) => {
        #[wasm_bindgen(js_name = $js_name)]
        pub unsafe fn $fn_name($($arg_name: $arg_ty),*) -> Vec<u8> {
            Player::write(&mut *POOL, &<$struct>::new($($arg_val),*))
        }
    };
}

static mut PLAYERS: Lazy<Vec<Option<Player>>> = Lazy::new(|| vec![None; 2048]);
static mut PLAYER_GRID: Lazy<HashMap<u32, Vec<i32>>> = Lazy::new(|| HashMap::with_capacity(2048));
static mut PLAYER_RENDERER: Lazy<PlayerRenderer> = Lazy::new(PlayerRenderer::new);
static mut PLAYER_INFO: Lazy<PlayerInfo> = Lazy::new(PlayerInfo::new);

static mut NPCS: Lazy<Vec<Option<Npc>>> = Lazy::new(|| vec![None; 8192]);
static mut NPC_RENDERER: Lazy<NpcRenderer> = Lazy::new(NpcRenderer::new);
static mut NPC_INFO: Lazy<NpcInfo> = Lazy::new(NpcInfo::new);

static mut ZONE_MAP: Lazy<ZoneMap> = Lazy::new(ZoneMap::new);
static mut WORD_PACK: Lazy<WordPack> = Lazy::new(WordPack::new);
static mut POOL: Lazy<PacketPool> = Lazy::new(PacketPool::new);

#[wasm_bindgen(js_name = computePlayer)]
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
    visibility: Visibility,
    active: bool,
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

        if coord.packed != player.coord.packed && (CoordGrid::zone(coord.x()) != CoordGrid::zone(player.coord.x()) || CoordGrid::zone(coord.z()) != CoordGrid::zone(player.coord.z()) || coord.y() != player.coord.y()) {
            ZONE_MAP.zone(player.coord.x(), player.coord.y(), player.coord.z()).remove_player(pid);
            ZONE_MAP.zone(coord.x(), coord.y(), coord.z()).add_player(pid);
        }

        player.coord = coord;
        player.origin = origin;
        player.tele = tele;
        player.jump = jump;
        player.run_dir = runDir;
        player.walk_dir = walkDir;
        player.visibility = visibility;
        player.active = active;
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

        PLAYER_RENDERER.compute_info(&player);
        PLAYER_GRID.entry(player.coord.packed).or_insert_with(Vec::new).push(pid);
    }
}

#[wasm_bindgen(js_name = playerInfo)]
pub unsafe fn player_info(pid: i32, dx: i32, dz: i32, rebuild: bool) -> Option<Vec<u8>> {
    if pid == -1 {
        return None;
    }

    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        let bytes: Vec<u8> = PLAYER_INFO.encode(
            &mut **addr_of_mut!(PLAYER_RENDERER),
            &**addr_of!(PLAYERS),
            &mut **addr_of_mut!(ZONE_MAP),
            &**addr_of!(PLAYER_GRID),
            player,
            dx,
            dz,
            rebuild,
        );
        let buf: &mut Packet = &mut *POOL.take(1 + 2 + bytes.len());
        buf.p1(ServerInternalProt::PLAYER_INFO as i32);
        buf.pos += 2;
        let start: usize = buf.pos;
        buf.pdata(&bytes, 0, bytes.len());
        buf.psize2((buf.pos - start) as u16);
        return unsafe { Some(buf.data.get_unchecked(0..buf.pos).to_vec()) };
    }

    return None;
}

#[wasm_bindgen(js_name = addPlayer)]
pub unsafe fn add_player(pid: i32) {
    if pid == -1 {
        return;
    }
    *PLAYERS.as_mut_ptr().add(pid as usize) = Some(Player::new(pid));
}

#[wasm_bindgen(js_name = removePlayer)]
pub unsafe fn remove_player(pid: i32) {
    if pid == -1 {
        return;
    }
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        // remove player from zone.
        ZONE_MAP.zone(player.coord.x(), player.coord.y(), player.coord.z()).remove_player(pid);
        for nid in player.build.npcs.iter() {
            if let Some(npc) = unsafe { &mut *NPCS.as_mut_ptr().add(nid as usize) } {
                npc.observers = (npc.observers - 1).max(0);
            }
        }
        player.build.cleanup();
    }
    PLAYER_RENDERER.removePermanent(pid);
    *PLAYERS.as_mut_ptr().add(pid as usize) = None;
}

#[wasm_bindgen(js_name = hasPlayer)]
pub unsafe fn has_player(pid: i32, other: i32) -> bool {
    if pid == -1 || other == -1 {
        return false;
    }
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        return player.build.players.contains(other);
    }
    return false;
}

#[wasm_bindgen(js_name = computeNpc)]
pub unsafe fn compute_npc(
    x: u16,
    y: u8,
    z: u16,
    nid: i32,
    ntype: i32,
    tele: bool,
    runDir: i8,
    walkDir: i8,
    active: bool,
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

        if coord.packed != npc.coord.packed && (CoordGrid::zone(coord.x()) != CoordGrid::zone(npc.coord.x()) || CoordGrid::zone(coord.z()) != CoordGrid::zone(npc.coord.z()) || coord.y() != npc.coord.y()) {
            ZONE_MAP.zone(npc.coord.x(), npc.coord.y(), npc.coord.z()).remove_npc(nid);
            ZONE_MAP.zone(coord.x(), coord.y(), coord.z()).add_npc(nid);
        }

        npc.ntype = ntype;
        npc.coord = coord;
        npc.tele = tele;
        npc.run_dir = runDir;
        npc.walk_dir = walkDir;
        npc.active = active;
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

        NPC_RENDERER.compute_info(&npc);
    }
}

#[wasm_bindgen(js_name = npcInfo)]
pub unsafe fn npc_info(pid: i32, dx: i32, dz: i32, rebuild: bool) -> Option<Vec<u8>> {
    if pid == -1 {
        return None;
    }

    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        let bytes: Vec<u8> = NPC_INFO.encode(
            &mut **addr_of_mut!(NPC_RENDERER),
            &mut **addr_of_mut!(NPCS),
            &mut **addr_of_mut!(ZONE_MAP),
            player,
            dx,
            dz,
            rebuild
        );
        let buf: &mut Packet = &mut *POOL.take(1 + 2 + bytes.len());
        buf.p1(ServerInternalProt::NPC_INFO as i32);
        buf.pos += 2;
        let start: usize = buf.pos;
        buf.pdata(&bytes, 0, bytes.len());
        buf.psize2((buf.pos - start) as u16);
        return unsafe { Some(buf.data.get_unchecked(0..buf.pos).to_vec()) };
    }

    return None;
}

#[wasm_bindgen(js_name = addNpc)]
pub unsafe fn add_npc(nid: i32, ntype: i32) {
    if nid == -1 || ntype == -1 {
        return;
    }
    *NPCS.as_mut_ptr().add(nid as usize) = Some(Npc::new(nid, ntype));
}

#[wasm_bindgen(js_name = removeNpc)]
pub unsafe fn remove_npc(nid: i32) {
    if nid == -1 {
        return;
    }
    if let Some(npc) = &*NPCS.as_ptr().add(nid as usize) {
        // remove npc from zone.
        ZONE_MAP.zone(npc.coord.x(), npc.coord.y(), npc.coord.z()).remove_npc(nid);
    }
    NPC_RENDERER.removePermanent(nid);
    *NPCS.as_mut_ptr().add(nid as usize) = None;
}

#[wasm_bindgen(js_name = hasNpc)]
pub unsafe fn has_npc(pid: i32, nid: i32) -> bool {
    if pid == -1 || nid == -1 {
        return false;
    }
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        return player.build.npcs.contains(nid);
    }
    return false;
}

#[wasm_bindgen(js_name = getNpcObservers)]
pub unsafe fn get_npc_observers(nid: i32) -> i32 {
    if nid == -1 {
        return 0;
    }
    if let Some(npc) = &*NPCS.as_ptr().add(nid as usize) {
        return npc.observers as i32;
    }
    return 0;
}

#[wasm_bindgen(js_name = cleanup)]
pub unsafe fn cleanup() {
    PLAYER_GRID.clear();
    PLAYER_RENDERER.removeTemporary();
    NPC_RENDERER.removeTemporary();
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

#[wasm_bindgen(js_name = cleanupPlayerBuildArea)]
pub unsafe fn cleanup_player_buildarea(pid: i32) {
    if pid == -1 {
        return;
    }
    if let Some(player) = &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        player.build.cleanup();
    }
}

// ---- encoders

buffer!(cam_lookat, "camLookAt", CamLookAt, (x: i32, z: i32, height: i32, speed: i32, multiplier: i32), (x, z, height, speed, multiplier));
buffer!(cam_moveto, "camMoveTo", CamMoveTo, (x: i32, z: i32, height: i32, speed: i32, multiplier: i32), (x, z, height, speed, multiplier));
buffer!(cam_reset, "camReset", CamReset, (), ());
buffer!(cam_shake, "camShake", CamShake, (shake: i32, jitter: i32, amplitude: i32, frequency: i32), (shake, jitter, amplitude, frequency));
buffer!(chat_filter_settings, "chatFilterSettings", ChatFilterSettings, (public: i32, private: i32, trade: i32), (public, private, trade));
buffer!(count_dialog, "countDialog", PCountDialog, (), ());
write!(data_land, "dataLand", DataLand, (x: i32, z: i32, offset: i32, length: i32, data: Vec<u8>), (x, z, offset, length, data));
write!(data_land_done, "dataLandDone", DataLandDone, (x: i32, z: i32), (x, z));
write!(data_loc, "dataLoc", DataLoc, (x: i32, z: i32, offset: i32, length: i32, data: Vec<u8>), (x, z, offset, length, data));
write!(data_loc_done, "dataLocDone", DataLocDone, (x: i32, z: i32), (x, z));
buffer!(enable_tracking, "enableTracking", EnableTracking, (), ());
buffer!(finish_tracking, "finishTracking", FinishTracking, (), ());
buffer!(hint_arrow, "hintArrow", HintArrow, (arrow: i32, nid: i32, pid2: i32, x: i32, z: i32, y: i32), (arrow, nid, pid2, x, z, y)); // todo: what should priority be?
buffer!(if_close, "ifClose", IfClose, (), ());
buffer!(if_open_chat, "ifOpenChat", IfOpenChat, (component: i32), (component));
buffer!(if_open_main, "ifOpenMain", IfOpenMain, (component: i32), (component));
buffer!(if_open_main_side, "ifOpenMainSide", IfOpenMainSide, (main: i32, side: i32), (main, side));
buffer!(if_open_side, "ifOpenSide", IfOpenSide, (component: i32), (component));
buffer!(if_setanim, "ifSetAnim", IfSetAnim, (component: i32, seq: i32), (component, seq));
buffer!(if_setcolour, "ifSetColour", IfSetColour, (component: i32, colour: i32), (component, colour));
buffer!(if_sethide, "ifSetHide", IfSetHide, (component: i32, hidden: bool), (component, hidden));
buffer!(if_setmodel, "ifSetModel", IfSetModel, (component: i32, model: i32), (component, model));
buffer!(if_setnpchead, "ifSetNpcHead", IfSetNpcHead, (component: i32, npc: i32), (component, npc));
buffer!(if_setobject, "ifSetObject", IfSetObject, (component: i32, obj: i32, scale: i32), (component, obj, scale));
buffer!(if_setplayerhead, "ifSetPlayerHead", IfSetPlayerHead, (component: i32), (component));
buffer!(if_setposition, "ifSetPosition", IfSetPosition, (component: i32, x: i32, y: i32), (component, x, y));
buffer!(if_setrecol, "ifSetRecol", IfSetRecol, (component: i32, src: i32, dst: i32), (component, src, dst));
buffer!(if_settab, "ifSetTab", IfSetTab, (component: i32, tab: i32), (component, tab));
buffer!(if_settabactive, "ifSetTabActive", IfSetTabActive, (tab: i32), (tab));
buffer!(if_settext, "ifSetText", IfSetText, (component: i32, text: String), (component, text));
buffer!(last_login_info, "lastLoginInfo", LastLoginInfo, (lastIp: i32, daysSinceLogin: i32, daysSinceRecovery: i32, messages: i32), (lastIp, daysSinceLogin, daysSinceRecovery, messages));
write!(loc_addchange, "locAddChange", LocAddChange, (coord: i32, loc: i32, shape: i32, angle: i32), (coord, loc, shape, angle));
write!(loc_anim, "locAnim", LocAnim, (coord: i32, shape: i32, angle: i32, seq: i32), (coord, shape, angle, seq));
write!(loc_del, "locDel", LocDel, (coord: i32, shape: i32, angle: i32), (coord, shape, angle));
write!(loc_merge, "locMerge", LocMerge, (srcX: i32, srcZ: i32, shape: i32, angle: i32, loc: i32, start: i32, end: i32, pid: i32, east: i32, south: i32, west: i32, north: i32), (srcX, srcZ, shape, angle, loc, start, end, pid, east, south, west, north));
write!(logout, "logout", Logout, (), ());
write!(map_anim, "mapAnim", MapAnim, (coord: i32, spotanim: i32, height: i32, delay: i32), (coord, spotanim, height, delay));
write!(map_projanim, "mapProjAnim", MapProjAnim, (srcX: i32, srcZ: i32, dstX: i32, dstZ: i32, target: i32, spotanim: i32, srcHeight: i32, dstHeight: i32, start: i32, end: i32, peak: i32, arc: i32), (srcX, srcZ, dstX, dstZ, target, spotanim, srcHeight, dstHeight, start, end, peak, arc));
write!(message_game, "messageGame", MessageGame, (msg: String), (msg));
write!(message_private_out, "messagePrivateOut", MessagePrivateOut, (from: i64, id: i32, staffModLevel: i32, msg: Vec<u8>), (from, id, staffModLevel, msg));
buffer!(midi_jingle, "midiJingle", MidiJingle, (delay: i32, data: Vec<u8>), (delay, data));
buffer!(midi_song, "midiSong", MidiSong, (name: String, crc: i32, length: i32), (name, crc, length));
write!(obj_add, "objAdd", ObjAdd, (coord: i32, obj: i32, count: i32), (coord, obj, count));
write!(obj_count, "objCount", ObjCount, (coord: i32, obj: i32, oldCount: i32, newCount: i32), (coord, obj, oldCount, newCount));
write!(obj_del, "objDel", ObjDel, (coord: i32, obj: i32), (coord, obj));
write!(obj_reveal, "objReveal", ObjReveal, (coord: i32, obj: i32, count: i32, receiver: i32), (coord, obj, count, receiver));
write!(rebuild_normal, "rebuildNormal", RebuildNormal, (x: i32, z: i32, squares: Vec<u16>, maps: Vec<i32>, locs: Vec<i32>), (x, z, squares, maps, locs));
write!(reset_anims, "resetAnims", ResetAnims, (), ()); // todo: what should priority be?
write!(reset_clientvarcache, "resetClientVarCache", ResetClientVarCache, (), ());
buffer!(set_multiway, "setMultiway", SetMultiway, (hidden: bool), (hidden));
buffer!(synth_sound, "synthSound", SynthSound, (synth: i32, loops: i32, delay: i32), (synth, loops, delay));
buffer!(tut_flash, "tutFlash", TutFlash, (tab: i32), (tab));
buffer!(tut_open, "tutOpen", TutOpen, (component: i32), (component));
write!(unset_map_flag, "unsetMapFlag", UnsetMapFlag, (), ());
buffer!(update_friendlist, "updateFriendList", UpdateFriendList, (name: i64, node: i32), (name, node));
buffer!(update_ignorelist, "updateIgnoreList", UpdateIgnoreList, (names: Vec<i64>), (names));
write!(update_inv_full, "updateInvFull", UpdateInvFull, (size: i32, component: i32, inv: Vec<i64>), (size, component, inv));
write!(update_inv_partial, "updateInvPartial", UpdateInvPartial, (component: i32, slots: Vec<i32>, inv: Vec<i64>), (component, slots, inv));
write!(update_inv_stop_transmit, "updateInvStopTransmit", UpdateInvStopTransmit, (component: i32), (component));
write!(update_pid, "updatePid", UpdatePid, (pid: i32), (pid)); // todo: what should priority be?
buffer!(update_reboot_timer, "updateRebootTimer", UpdateRebootTimer, (ticks: i32), (ticks)); // todo: what should priority be?
buffer!(update_runenergy, "updateRunEnergy", UpdateRunEnergy, (energy: i32), (energy));
buffer!(update_runweight, "updateRunWeight", UpdateRunWeight, (kg: i32), (kg));
buffer!(update_stat, "updateStat", UpdateStat, (stat: i32, experience: i32, level: i32), (stat, experience, level));
write!(update_zone_full_follows, "updateZoneFullFollows", UpdateZoneFullFollows, (x: i32, z: i32, originX: i32, originZ: i32), (x, z, originX, originZ));
write!(update_zone_partial_enclosed, "updateZonePartialEnclosed", UpdateZonePartialEnclosed, (x: i32, z: i32, originX: i32, originZ: i32, data: Vec<u8>), (x, z, originX, originZ, data));
write!(update_zone_partial_follows, "updateZonePartialFollows", UpdateZonePartialFollows, (x: i32, z: i32, originX: i32, originZ: i32), (x, z, originX, originZ));
write!(varp_small, "varpSmall", VarpSmall, (id: i32, value: i32), (id, value));
write!(varp_large, "varpLarge", VarpLarge, (id: i32, value: i32), (id, value));

// ---- decoders

read!(client_cheat, "clientCheat", ClientCheat, ClientProt::CLIENT_CHEAT);
read!(close_modal, "closeModal", CloseModal, ClientProt::CLOSE_MODAL);
read!(friend_list_add, "friendListAdd", FriendListAdd, ClientProt::FRIENDLIST_ADD);
read!(friend_list_del, "friendListDel", FriendListDel, ClientProt::FRIENDLIST_DEL);
read!(idle_timer, "idleTimer", IdleTimer, ClientProt::IDLE_TIMER);
read!(if_button, "ifButton", IfButton, ClientProt::IF_BUTTON);
read!(if_player_design, "ifPlayerDesign", IfPlayerDesign, ClientProt::IF_PLAYERDESIGN);
read!(ignore_list_add, "ignoreListAdd", IgnoreListAdd, ClientProt::IGNORELIST_ADD);
read!(ignore_list_del, "ignoreListDel", IgnoreListDel, ClientProt::IGNORELIST_DEL);
read!(inv_button1, "invButton1", InvButton, ClientProt::INV_BUTTON1);
read!(inv_button2, "invButton2", InvButton, ClientProt::INV_BUTTON2);
read!(inv_button3, "invButton3", InvButton, ClientProt::INV_BUTTON3);
read!(inv_button4, "invButton4", InvButton, ClientProt::INV_BUTTON4);
read!(inv_button5, "invButton5", InvButton, ClientProt::INV_BUTTON5);
read!(inv_button_d, "invButtonD", InvButtonD, ClientProt::INV_BUTTOND);
read!(message_private, "messagePrivate", MessagePrivate, ClientProt::MESSAGE_PRIVATE);
read!(message_public, "messagePublic", MessagePublic, ClientProt::MESSAGE_PUBLIC);
read!(move_minimap_click, "moveMinimapClick", MoveClick, ClientProt::MOVE_MINIMAPCLICK);
read!(move_game_click, "moveGameClick", MoveClick, ClientProt::MOVE_GAMECLICK);
read!(move_op_click, "moveOpClick", MoveClick, ClientProt::MOVE_OPCLICK);
read!(no_timeout, "noTimeout", NoTimeout, ClientProt::NO_TIMEOUT);
read!(opheld1, "opheld1", OpHeld, ClientProt::OPHELD1);
read!(opheld2, "opheld2", OpHeld, ClientProt::OPHELD2);
read!(opheld3, "opheld3", OpHeld, ClientProt::OPHELD3);
read!(opheld4, "opheld4", OpHeld, ClientProt::OPHELD4);
read!(opheld5, "opheld5", OpHeld, ClientProt::OPHELD5);
read!(opheld_t, "opheldT", OpHeldT, ClientProt::OPHELDT);
read!(opheld_u, "opheldU", OpHeldU, ClientProt::OPHELDU);
read!(oploc1, "oploc1", OpLoc, ClientProt::OPLOC1);
read!(oploc2, "oploc2", OpLoc, ClientProt::OPLOC2);
read!(oploc3, "oploc3", OpLoc, ClientProt::OPLOC3);
read!(oploc4, "oploc4", OpLoc, ClientProt::OPLOC4);
read!(oploc5, "oploc5", OpLoc, ClientProt::OPLOC5);
read!(oploc_t, "oplocT", OpLocT, ClientProt::OPLOCT);
read!(oploc_u, "oplocU", OpLocU, ClientProt::OPLOCU);
read!(opnpc1, "opnpc1", OpNpc, ClientProt::OPNPC1);
read!(opnpc2, "opnpc2", OpNpc, ClientProt::OPNPC2);
read!(opnpc3, "opnpc3", OpNpc, ClientProt::OPNPC3);
read!(opnpc4, "opnpc4", OpNpc, ClientProt::OPNPC4);
read!(opnpc5, "opnpc5", OpNpc, ClientProt::OPNPC5);
read!(opnpc_t, "opnpcT", OpNpcT, ClientProt::OPNPCT);
read!(opnpc_u, "opnpcU", OpNpcU, ClientProt::OPNPCU);
read!(opobj1, "opobj1", OpObj, ClientProt::OPOBJ1);
read!(opobj2, "opobj2", OpObj, ClientProt::OPOBJ2);
read!(opobj3, "opobj3", OpObj, ClientProt::OPOBJ3);
read!(opobj4, "opobj4", OpObj, ClientProt::OPOBJ4);
read!(opobj5, "opobj5", OpObj, ClientProt::OPOBJ5);
read!(opobj_t, "opobjT", OpObjT, ClientProt::OPOBJT);
read!(opobj_u, "opobjU", OpObjU, ClientProt::OPOBJU);
read!(opplayer1, "opplayer1", OpPlayer, ClientProt::OPPLAYER1);
read!(opplayer2, "opplayer2", OpPlayer, ClientProt::OPPLAYER2);
read!(opplayer3, "opplayer3", OpPlayer, ClientProt::OPPLAYER3);
read!(opplayer4, "opplayer4", OpPlayer, ClientProt::OPPLAYER4);
read!(opplayer_t, "opplayerT", OpPlayerT, ClientProt::OPPLAYERT);
read!(opplayer_u, "opplayerU", OpPlayerU, ClientProt::OPPLAYERU);
read!(rebuild_getmaps, "rebuildGetMaps", RebuildGetMaps, ClientProt::REBUILD_GETMAPS);
read!(resume_pausebutton, "resumePauseButton", ResumePauseButton, ClientProt::RESUME_PAUSEBUTTON);
read!(resume_countdialog, "resumeCountDialog", ResumePCountDialog, ClientProt::RESUME_P_COUNTDIALOG);
read!(tutorial_clickside, "tutorialClickSide", TutorialClickSide, ClientProt::TUTORIAL_CLICKSIDE);
read!(chat_setmode, "chatSetMode", ChatSetMode, ClientProt::CHAT_SETMODE);
read!(event_tracking, "eventTracking", EventTracking, ClientProt::EVENT_TRACKING);
read!(report_abuse, "reportAbuse", ReportAbuse, ClientProt::REPORT_ABUSE);
read!(event_camera_position, "eventCameraPosition", EventCameraPosition, ClientProt::EVENT_CAMERA_POSITION);
read!(anticheatop1, "anticheatOp1", AnticheatOp1, ClientProt::ANTICHEAT_OPLOGIC1);
read!(anticheatop2, "anticheatOp2", AnticheatOp2, ClientProt::ANTICHEAT_OPLOGIC2);
read!(anticheatop3, "anticheatOp3", AnticheatOp3, ClientProt::ANTICHEAT_OPLOGIC3);
read!(anticheatop4, "anticheatOp4", AnticheatOp4, ClientProt::ANTICHEAT_OPLOGIC4);
read!(anticheatop5, "anticheatOp5", AnticheatOp5, ClientProt::ANTICHEAT_OPLOGIC5);
read!(anticheatop6, "anticheatOp6", AnticheatOp6, ClientProt::ANTICHEAT_OPLOGIC6);
read!(anticheatop7, "anticheatOp7", AnticheatOp7, ClientProt::ANTICHEAT_OPLOGIC7);
read!(anticheatop8, "anticheatOp8", AnticheatOp8, ClientProt::ANTICHEAT_OPLOGIC8);
read!(anticheatop9, "anticheatOp9", AnticheatOp9, ClientProt::ANTICHEAT_OPLOGIC9);
read!(anticheatcycle1, "anticheatCycle1", AnticheatCycle1, ClientProt::ANTICHEAT_CYCLELOGIC1);
read!(anticheatcycle2, "anticheatCycle2", AnticheatCycle2, ClientProt::ANTICHEAT_CYCLELOGIC2);
read!(anticheatcycle3, "anticheatCycle3", AnticheatCycle3, ClientProt::ANTICHEAT_CYCLELOGIC3);
read!(anticheatcycle4, "anticheatCycle4", AnticheatCycle4, ClientProt::ANTICHEAT_CYCLELOGIC4);
read!(anticheatcycle5, "anticheatCycle5", AnticheatCycle5, ClientProt::ANTICHEAT_CYCLELOGIC5);
read!(anticheatcycle6, "anticheatCycle6", AnticheatCycle6, ClientProt::ANTICHEAT_CYCLELOGIC6);

// ---- misc

static PACKET_LOOKUP: Lazy<Vec<Option<IncomingPacket>>> = Lazy::new(|| {
    let mut lookup: Vec<Option<IncomingPacket>> = vec![None; 255];
    lookup[ClientInternalProt::CLIENT_CHEAT as usize] = Some(IncomingPacket::new(ClientProt::CLIENT_CHEAT as i32, ClientCheat::length()));
    lookup[ClientInternalProt::CLOSE_MODAL as usize] = Some(IncomingPacket::new(ClientProt::CLOSE_MODAL as i32, CloseModal::length()));
    lookup[ClientInternalProt::FRIENDLIST_ADD as usize] = Some(IncomingPacket::new(ClientProt::FRIENDLIST_ADD as i32, FriendListAdd::length()));
    lookup[ClientInternalProt::FRIENDLIST_DEL as usize] = Some(IncomingPacket::new(ClientProt::FRIENDLIST_DEL as i32, FriendListDel::length()));
    lookup[ClientInternalProt::IDLE_TIMER as usize] = Some(IncomingPacket::new(ClientProt::IDLE_TIMER as i32, IdleTimer::length()));
    lookup[ClientInternalProt::IF_BUTTON as usize] = Some(IncomingPacket::new(ClientProt::IF_BUTTON as i32, IfButton::length()));
    lookup[ClientInternalProt::IF_PLAYERDESIGN as usize] = Some(IncomingPacket::new(ClientProt::IF_PLAYERDESIGN as i32, IfPlayerDesign::length()));
    lookup[ClientInternalProt::IGNORELIST_ADD as usize] = Some(IncomingPacket::new(ClientProt::IGNORELIST_ADD as i32, IgnoreListAdd::length()));
    lookup[ClientInternalProt::IGNORELIST_DEL as usize] = Some(IncomingPacket::new(ClientProt::IGNORELIST_DEL as i32, IgnoreListDel::length()));
    lookup[ClientInternalProt::INV_BUTTON1 as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTON1 as i32, InvButton::length()));
    lookup[ClientInternalProt::INV_BUTTON2 as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTON2 as i32, InvButton::length()));
    lookup[ClientInternalProt::INV_BUTTON3 as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTON3 as i32, InvButton::length()));
    lookup[ClientInternalProt::INV_BUTTON4 as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTON4 as i32, InvButton::length()));
    lookup[ClientInternalProt::INV_BUTTON5 as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTON5 as i32, InvButton::length()));
    lookup[ClientInternalProt::INV_BUTTOND as usize] = Some(IncomingPacket::new(ClientProt::INV_BUTTOND as i32, InvButtonD::length()));
    lookup[ClientInternalProt::MESSAGE_PRIVATE as usize] = Some(IncomingPacket::new(ClientProt::MESSAGE_PRIVATE as i32, MessagePrivate::length()));
    lookup[ClientInternalProt::MESSAGE_PUBLIC as usize] = Some(IncomingPacket::new(ClientProt::MESSAGE_PUBLIC as i32, MessagePublic::length()));
    lookup[ClientInternalProt::MOVE_MINIMAPCLICK as usize] = Some(IncomingPacket::new(ClientProt::MOVE_MINIMAPCLICK as i32, MoveClick::length()));
    lookup[ClientInternalProt::MOVE_GAMECLICK as usize] = Some(IncomingPacket::new(ClientProt::MOVE_GAMECLICK as i32, MoveClick::length()));
    lookup[ClientInternalProt::MOVE_OPCLICK as usize] = Some(IncomingPacket::new(ClientProt::MOVE_OPCLICK as i32, MoveClick::length()));
    lookup[ClientInternalProt::NO_TIMEOUT as usize] = Some(IncomingPacket::new(ClientProt::NO_TIMEOUT as i32, NoTimeout::length()));
    lookup[ClientInternalProt::OPHELD1 as usize] = Some(IncomingPacket::new(ClientProt::OPHELD1 as i32, OpHeld::length()));
    lookup[ClientInternalProt::OPHELD2 as usize] = Some(IncomingPacket::new(ClientProt::OPHELD2 as i32, OpHeld::length()));
    lookup[ClientInternalProt::OPHELD3 as usize] = Some(IncomingPacket::new(ClientProt::OPHELD3 as i32, OpHeld::length()));
    lookup[ClientInternalProt::OPHELD4 as usize] = Some(IncomingPacket::new(ClientProt::OPHELD4 as i32, OpHeld::length()));
    lookup[ClientInternalProt::OPHELD5 as usize] = Some(IncomingPacket::new(ClientProt::OPHELD5 as i32, OpHeld::length()));
    lookup[ClientInternalProt::OPHELDT as usize] = Some(IncomingPacket::new(ClientProt::OPHELDT as i32, OpHeldT::length()));
    lookup[ClientInternalProt::OPHELDU as usize] = Some(IncomingPacket::new(ClientProt::OPHELDU as i32, OpHeldU::length()));
    lookup[ClientInternalProt::OPLOC1 as usize] = Some(IncomingPacket::new(ClientProt::OPLOC1 as i32, OpLoc::length()));
    lookup[ClientInternalProt::OPLOC2 as usize] = Some(IncomingPacket::new(ClientProt::OPLOC2 as i32, OpLoc::length()));
    lookup[ClientInternalProt::OPLOC3 as usize] = Some(IncomingPacket::new(ClientProt::OPLOC3 as i32, OpLoc::length()));
    lookup[ClientInternalProt::OPLOC4 as usize] = Some(IncomingPacket::new(ClientProt::OPLOC4 as i32, OpLoc::length()));
    lookup[ClientInternalProt::OPLOC5 as usize] = Some(IncomingPacket::new(ClientProt::OPLOC5 as i32, OpLoc::length()));
    lookup[ClientInternalProt::OPLOCT as usize] = Some(IncomingPacket::new(ClientProt::OPLOCT as i32, OpLocT::length()));
    lookup[ClientInternalProt::OPLOCU as usize] = Some(IncomingPacket::new(ClientProt::OPLOCU as i32, OpLocU::length()));
    lookup[ClientInternalProt::OPNPC1 as usize] = Some(IncomingPacket::new(ClientProt::OPNPC1 as i32, OpNpc::length()));
    lookup[ClientInternalProt::OPNPC2 as usize] = Some(IncomingPacket::new(ClientProt::OPNPC2 as i32, OpNpc::length()));
    lookup[ClientInternalProt::OPNPC3 as usize] = Some(IncomingPacket::new(ClientProt::OPNPC3 as i32, OpNpc::length()));
    lookup[ClientInternalProt::OPNPC4 as usize] = Some(IncomingPacket::new(ClientProt::OPNPC4 as i32, OpNpc::length()));
    lookup[ClientInternalProt::OPNPC5 as usize] = Some(IncomingPacket::new(ClientProt::OPNPC5 as i32, OpNpc::length()));
    lookup[ClientInternalProt::OPNPCT as usize] = Some(IncomingPacket::new(ClientProt::OPNPCT as i32, OpNpcT::length()));
    lookup[ClientInternalProt::OPNPCU as usize] = Some(IncomingPacket::new(ClientProt::OPNPCU as i32, OpNpcU::length()));
    lookup[ClientInternalProt::OPOBJ1 as usize] = Some(IncomingPacket::new(ClientProt::OPOBJ1 as i32, OpObj::length()));
    lookup[ClientInternalProt::OPOBJ2 as usize] = Some(IncomingPacket::new(ClientProt::OPOBJ2 as i32, OpObj::length()));
    lookup[ClientInternalProt::OPOBJ3 as usize] = Some(IncomingPacket::new(ClientProt::OPOBJ3 as i32, OpObj::length()));
    lookup[ClientInternalProt::OPOBJ4 as usize] = Some(IncomingPacket::new(ClientProt::OPOBJ4 as i32, OpObj::length()));
    lookup[ClientInternalProt::OPOBJ5 as usize] = Some(IncomingPacket::new(ClientProt::OPOBJ5 as i32, OpObj::length()));
    lookup[ClientInternalProt::OPOBJT as usize] = Some(IncomingPacket::new(ClientProt::OPOBJT as i32, OpObjT::length()));
    lookup[ClientInternalProt::OPOBJU as usize] = Some(IncomingPacket::new(ClientProt::OPOBJU as i32, OpObjU::length()));
    lookup[ClientInternalProt::OPPLAYER1 as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYER1 as i32, OpPlayer::length()));
    lookup[ClientInternalProt::OPPLAYER2 as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYER2 as i32, OpPlayer::length()));
    lookup[ClientInternalProt::OPPLAYER3 as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYER3 as i32, OpPlayer::length()));
    lookup[ClientInternalProt::OPPLAYER4 as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYER4 as i32, OpPlayer::length()));
    lookup[ClientInternalProt::OPPLAYERT as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYERT as i32, OpPlayerT::length()));
    lookup[ClientInternalProt::OPPLAYERU as usize] = Some(IncomingPacket::new(ClientProt::OPPLAYERU as i32, OpPlayerU::length()));
    lookup[ClientInternalProt::REBUILD_GETMAPS as usize] = Some(IncomingPacket::new(ClientProt::REBUILD_GETMAPS as i32, RebuildGetMaps::length()));
    lookup[ClientInternalProt::RESUME_PAUSEBUTTON as usize] = Some(IncomingPacket::new(ClientProt::RESUME_PAUSEBUTTON as i32, ResumePauseButton::length()));
    lookup[ClientInternalProt::RESUME_P_COUNTDIALOG as usize] = Some(IncomingPacket::new(ClientProt::RESUME_P_COUNTDIALOG as i32, ResumePCountDialog::length()));
    lookup[ClientInternalProt::TUTORIAL_CLICKSIDE as usize] = Some(IncomingPacket::new(ClientProt::TUTORIAL_CLICKSIDE as i32, TutorialClickSide::length()));
    lookup[ClientInternalProt::CHAT_SETMODE as usize] = Some(IncomingPacket::new(ClientProt::CHAT_SETMODE as i32, ChatSetMode::length()));
    lookup[ClientInternalProt::EVENT_TRACKING as usize] = Some(IncomingPacket::new(ClientProt::EVENT_TRACKING as i32, EventTracking::length()));
    lookup[ClientInternalProt::REPORT_ABUSE as usize] = Some(IncomingPacket::new(ClientProt::REPORT_ABUSE as i32, ReportAbuse::length()));
    lookup[ClientInternalProt::EVENT_CAMERA_POSITION as usize] = Some(IncomingPacket::new(ClientProt::EVENT_CAMERA_POSITION as i32, EventCameraPosition::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC1 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC1 as i32, AnticheatOp1::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC2 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC2 as i32, AnticheatOp2::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC3 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC3 as i32, AnticheatOp3::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC4 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC4 as i32, AnticheatOp4::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC5 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC5 as i32, AnticheatOp5::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC6 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC6 as i32, AnticheatOp6::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC7 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC7 as i32, AnticheatOp7::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC8 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC8 as i32, AnticheatOp8::length()));
    lookup[ClientInternalProt::ANTICHEAT_OPLOGIC9 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC9 as i32, AnticheatOp9::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC1 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC1 as i32, AnticheatCycle1::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC2 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC2 as i32, AnticheatCycle2::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC3 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC3 as i32, AnticheatCycle3::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC4 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC4 as i32, AnticheatCycle4::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC5 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC5 as i32, AnticheatCycle5::length()));
    lookup[ClientInternalProt::ANTICHEAT_CYCLELOGIC6 as usize] = Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC6 as i32, AnticheatCycle6::length()));
    return lookup;
});

#[wasm_bindgen(js_name = nextBufferedWrite)]
pub unsafe fn next_buffered_write(pid: i32) -> Option<Vec<u8>> {
    if pid == -1 {
        return None;
    }
    return match &mut *PLAYERS.as_mut_ptr().add(pid as usize) {
        None => None,
        Some(player) => player.write_queue.pop_front(),
    };
}

#[wasm_bindgen(js_name = nextBufferedRead)]
pub unsafe fn next_buffered_read(id: i32) -> i16 {
    return match &*PACKET_LOOKUP.as_ptr().add(id as usize) {
        None => -1,
        Some(packet) => ((packet.id as i16) << 8) | (packet.length as i16 & 0xff),
    }
}

#[wasm_bindgen(js_name = isBufferFull)]
pub unsafe fn is_buffer_full(pid: i32) -> bool {
    if pid == -1 {
        return false;
    }
    return match &*PLAYERS.as_ptr().add(pid as usize) {
        None => false,
        Some(player) => player
            .write_queue
            .iter()
            .map(|packet| packet.len())
            .scan(0, |acc, len| {
                *acc += len;
                return if *acc >= 5000 {
                    None
                } else {
                    Some(*acc)
                }
            })
            .last()
            .unwrap_or(0) >= 5000
    };
}

#[wasm_bindgen(js_name = unpackWords)]
pub unsafe fn unpack_words(bytes: Vec<u8>) -> String {
    return WORD_PACK.unpack(Packet::from(bytes));
}

#[wasm_bindgen(js_name = packWords)]
pub unsafe fn pack_words(msg: String) -> Vec<u8> {
    return WORD_PACK.pack(msg);
}
