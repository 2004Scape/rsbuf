#![allow(non_snake_case)]

use crate::coord::CoordGrid;
use crate::grid::ZoneMap;
use crate::message::{IncomingPacket, MessageDecoder, OutgoingPacket};
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
use crate::pack::WordPack;
use crate::packet::Packet;
use crate::player::{Chat, ExactMove, Player};
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
use once_cell::sync::Lazy;
use out::player_info::PlayerInfo;
use std::collections::HashMap;
use std::ptr::{addr_of, addr_of_mut};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod packet;
pub mod renderer;
pub mod build;

mod coord;
mod player;
mod prot;
mod message;
mod grid;
mod npc;
mod visibility;
mod priority;
mod category;
mod r#in;
mod out;
mod pack;

static mut PLAYERS: Lazy<Vec<Option<Player>>> = Lazy::new(|| vec![None; 2048]);
static mut PLAYER_GRID: Lazy<HashMap<u32, Vec<i32>>> = Lazy::new(|| HashMap::with_capacity(2048));
static mut PLAYER_RENDERER: Lazy<PlayerRenderer> = Lazy::new(PlayerRenderer::new);
static mut PLAYER_INFO: Lazy<PlayerInfo> = Lazy::new(PlayerInfo::new);

static mut NPCS: Lazy<Vec<Option<Npc>>> = Lazy::new(|| vec![None; 8192]);
static mut NPC_RENDERER: Lazy<NpcRenderer> = Lazy::new(NpcRenderer::new);
static mut NPC_INFO: Lazy<NpcInfo> = Lazy::new(NpcInfo::new);

static mut ZONE_MAP: Lazy<ZoneMap> = Lazy::new(ZoneMap::new);

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
pub unsafe fn player_info(pos: usize, pid: i32, dx: i32, dz: i32, rebuild: bool) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return Some(OutgoingPacket::new(
            Some(PLAYER_INFO.encode(
                pos,
                &mut **addr_of_mut!(PLAYER_RENDERER),
                &**addr_of!(PLAYERS),
                &mut **addr_of_mut!(ZONE_MAP),
                &**addr_of!(PLAYER_GRID),
                player,
                dx,
                dz,
                rebuild,
            )),
            ServerInternalProt::PLAYER_INFO as i32,
            -2,
        ));
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
pub unsafe fn npc_info(pos: usize, pid: i32, dx: i32, dz: i32, rebuild: bool) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return Some(OutgoingPacket::new(
            Some(NPC_INFO.encode(
                pos,
                &mut **addr_of_mut!(NPC_RENDERER),
                &mut **addr_of_mut!(NPCS),
                &mut **addr_of_mut!(ZONE_MAP),
                player,
                dx,
                dz,
                rebuild
            )),
            ServerInternalProt::NPC_INFO as i32,
            -2,
        ));
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

#[wasm_bindgen(js_name = camLookAt)]
pub unsafe fn cam_lookat(pid: i32, x: i32, z: i32, height: i32, speed: i32, multiplier: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&CamLookAt::new(x, z, height, speed, multiplier));
    }

    return None;
}

#[wasm_bindgen(js_name = camMoveTo)]
pub unsafe fn cam_moveto(pid: i32, x: i32, z: i32, height: i32, speed: i32, multiplier: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&CamMoveTo::new(x, z, height, speed, multiplier));
    }

    return None;
}

#[wasm_bindgen(js_name = camReset)]
pub unsafe fn cam_reset(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&CamReset::new());
    }

    return None;
}

#[wasm_bindgen(js_name = camShake)]
pub unsafe fn cam_shake(pid: i32, shake: i32, jitter: i32, amplitude: i32, frequency: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&CamShake::new(shake, jitter, amplitude, frequency));
    }

    return None;
}

#[wasm_bindgen(js_name = chatFilterSettings)]
pub unsafe fn chat_filter_settings(pid: i32, public: i32, private: i32, trade: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&ChatFilterSettings::new(public, private, trade));
    }

    return None;
}

#[wasm_bindgen(js_name = countDialog)]
pub unsafe fn count_dialog(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&PCountDialog::new());
    }

    return None;
}

#[wasm_bindgen(js_name = dataLand)]
pub unsafe fn data_land(pid: i32, x: i32, z: i32, offset: i32, length: i32, data: Vec<u8>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&DataLand::new(x, z, offset, length, data));
    }

    return None;
}

#[wasm_bindgen(js_name = dataLandDone)]
pub unsafe fn data_land_done(pid: i32, x: i32, z: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&DataLandDone::new(x, z));
    }

    return None;
}

#[wasm_bindgen(js_name = dataLoc)]
pub unsafe fn data_loc(pid: i32, x: i32, z: i32, offset: i32, length: i32, data: Vec<u8>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&DataLoc::new(x, z, offset, length, data));
    }

    return None;
}

#[wasm_bindgen(js_name = dataLocDone)]
pub unsafe fn data_loc_done(pid: i32, x: i32, z: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&DataLocDone::new(x, z));
    }

    return None;
}

#[wasm_bindgen(js_name = enableTracking)]
pub unsafe fn enable_tracking(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&EnableTracking::new());
    }

    return None;
}

#[wasm_bindgen(js_name = finishTracking)]
pub unsafe fn finish_tracking(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&FinishTracking::new());
    }

    return None;
}

#[wasm_bindgen(js_name = hintArrow)]
pub unsafe fn hint_arrow(pid: i32, arrow: i32, nid: i32, pid2: i32, x: i32, z: i32, y: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&HintArrow::new(arrow, nid, pid2, x, z, y));
    }

    return None;
}

#[wasm_bindgen(js_name = ifClose)]
pub unsafe fn if_close(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfClose::new());
    }

    return None;
}

#[wasm_bindgen(js_name = ifOpenChat)]
pub unsafe fn if_open_chat(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfOpenChat::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = ifOpenMain)]
pub unsafe fn if_open_main(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfOpenMain::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = ifOpenMainSide)]
pub unsafe fn if_open_main_side(pid: i32, main: i32, side: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfOpenMainSide::new(main, side));
    }

    return None;
}

#[wasm_bindgen(js_name = ifOpenSide)]
pub unsafe fn if_open_side(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfOpenSide::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetAnim)]
pub unsafe fn if_setanim(pid: i32, component: i32, seq: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetAnim::new(component, seq));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetColour)]
pub unsafe fn if_setcolour(pid: i32, component: i32, colour: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetColour::new(component, colour));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetHide)]
pub unsafe fn if_sethide(pid: i32, component: i32, hidden: bool) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetHide::new(component, hidden));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetModel)]
pub unsafe fn if_setmodel(pid: i32, component: i32, model: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetModel::new(component, model));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetNpcHead)]
pub unsafe fn if_setnpchead(pid: i32, component: i32, npc: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetNpcHead::new(component, npc));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetObject)]
pub unsafe fn if_setobject(pid: i32, component: i32, obj: i32, scale: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetObject::new(component, obj, scale));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetPlayerHead)]
pub unsafe fn if_setplayerhead(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetPlayerHead::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetPosition)]
pub unsafe fn if_setposition(pid: i32, component: i32, x: i32, y: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetPosition::new(component, x, y));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetRecol)]
pub unsafe fn if_setrecol(pid: i32, component: i32, src: i32, dst: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetRecol::new(component, src, dst));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetTab)]
pub unsafe fn if_settab(pid: i32, component: i32, tab: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetTab::new(component, tab));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetTabActive)]
pub unsafe fn if_settabactive(pid: i32, tab: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetTabActive::new(tab));
    }

    return None;
}

#[wasm_bindgen(js_name = ifSetText)]
pub unsafe fn if_settext(pid: i32, component: i32, text: String) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&IfSetText::new(component, text));
    }

    return None;
}

#[wasm_bindgen(js_name = lastLoginInfo)]
pub unsafe fn last_login_info(pid: i32, lastIp: i32, daysSinceLogin: i32, daysSinceRecovery: i32, messages: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&LastLoginInfo::new(lastIp, daysSinceLogin, daysSinceRecovery, messages));
    }

    return None;
}

#[wasm_bindgen(js_name = locAddChange)]
pub unsafe fn loc_addchange(coord: i32, loc: i32, shape: i32, angle: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&LocAddChange::new(coord, loc, shape, angle), enclose);
}

#[wasm_bindgen(js_name = locAnim)]
pub unsafe fn loc_anim(coord: i32, shape: i32, angle: i32, seq: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&LocAnim::new(coord, shape, angle, seq), enclose);
}

#[wasm_bindgen(js_name = locDel)]
pub unsafe fn loc_del(coord: i32, shape: i32, angle: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&LocDel::new(coord, shape, angle), enclose);
}

#[wasm_bindgen(js_name = locMerge)]
pub unsafe fn loc_merge(srcX: i32, srcZ: i32, shape: i32, angle: i32, loc: i32, start: i32, end: i32, pid: i32, east: i32, south: i32, west: i32, north: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&LocMerge::new(srcX, srcZ, shape, angle, loc, start, end, pid, east, south, west, north), enclose);
}

#[wasm_bindgen(js_name = logout)]
pub unsafe fn logout(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&Logout::new());
    }

    return None;
}

#[wasm_bindgen(js_name = mapAnim)]
pub unsafe fn map_anim(coord: i32, spotanim: i32, height: i32, delay: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&MapAnim::new(coord, spotanim, height, delay), enclose);
}

#[wasm_bindgen(js_name = mapProjAnim)]
pub unsafe fn map_projanim(srcX: i32, srcZ: i32, dstX: i32, dstZ: i32, target: i32, spotanim: i32, srcHeight: i32, dstHeight: i32, start: i32, end: i32, peak: i32, arc: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&MapProjAnim::new(srcX, srcZ, dstX, dstZ, target, spotanim, srcHeight, dstHeight, start, end, peak, arc), enclose);
}

#[wasm_bindgen(js_name = messageGame)]
pub unsafe fn message_game(pid: i32, msg: String) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&MessageGame::new(msg));
    }

    return None;
}

#[wasm_bindgen(js_name = messagePrivateOut)]
pub unsafe fn message_private_out(pid: i32, from: i64, id: i32, staffModLevel: i32, msg: String) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&MessagePrivateOut::new(from, id, staffModLevel, msg));
    }

    return None;
}

#[wasm_bindgen(js_name = midiJingle)]
pub unsafe fn midi_jingle(pid: i32, delay: i32, data: Vec<u8>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&MidiJingle::new(delay, data));
    }

    return None;
}

#[wasm_bindgen(js_name = midiSong)]
pub unsafe fn midi_song(pid: i32, name: String, crc: i32, length: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&MidiSong::new(name, crc, length));
    }

    return None;
}

#[wasm_bindgen(js_name = objAdd)]
pub unsafe fn obj_add(coord: i32, obj: i32, count: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&ObjAdd::new(coord, obj, count), enclose);
}

#[wasm_bindgen(js_name = objCount)]
pub unsafe fn obj_count(coord: i32, obj: i32, oldCount: i32, newCount: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&ObjCount::new(coord, obj, oldCount, newCount), enclose);
}

#[wasm_bindgen(js_name = objDel)]
pub unsafe fn obj_del(coord: i32, obj: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&ObjDel::new(coord, obj), enclose);
}

#[wasm_bindgen(js_name = objReveal)]
pub unsafe fn obj_reveal(coord: i32, obj: i32, count: i32, receiver: i32, enclose: bool) -> Option<OutgoingPacket> {
    return Player::write_zone_message(&ObjReveal::new(coord, obj, count, receiver), enclose);
}

#[wasm_bindgen(js_name = rebuildNormal)]
pub unsafe fn rebuild_normal(pid: i32, x: i32, z: i32, squares: Vec<u16>, maps: Vec<i32>, locs: Vec<i32>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&RebuildNormal::new(x, z, squares, maps, locs));
    }

    return None;
}

#[wasm_bindgen(js_name = resetAnims)]
pub unsafe fn reset_anims(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&ResetAnims::new());
    }

    return None;
}

#[wasm_bindgen(js_name = resetClientVarCache)]
pub unsafe fn reset_clientvarcache(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&ResetClientVarCache::new());
    }

    return None;
}

#[wasm_bindgen(js_name = setMultiway)]
pub unsafe fn set_multiway(pid: i32, hidden: bool) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&SetMultiway::new(hidden));
    }

    return None;
}

#[wasm_bindgen(js_name = synthSound)]
pub unsafe fn synth_sound(pid: i32, synth: i32, loops: i32, delay: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&SynthSound::new(synth, loops, delay));
    }

    return None;
}

#[wasm_bindgen(js_name = tutFlash)]
pub unsafe fn tut_flash(pid: i32, tab: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&TutFlash::new(tab));
    }

    return None;
}

#[wasm_bindgen(js_name = tutOpen)]
pub unsafe fn tut_open(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&TutOpen::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = unsetMapFlag)]
pub unsafe fn unset_map_flag(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UnsetMapFlag::new());
    }

    return None;
}

#[wasm_bindgen(js_name = updateFriendList)]
pub unsafe fn update_friendlist(pid: i32, name: i64, node: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateFriendList::new(name, node));
    }

    return None;
}

#[wasm_bindgen(js_name = updateIgnoreList)]
pub unsafe fn update_ignorelist(pid: i32, names: Vec<i64>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateIgnoreList::new(names));
    }

    return None;
}

#[wasm_bindgen(js_name = updateInvFull)]
pub unsafe fn update_inv_full(pid: i32, size: i32, component: i32, objs: Vec<i64>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateInvFull::new(size, component, objs));
    }

    return None;
}

#[wasm_bindgen(js_name = updateInvPartial)]
pub unsafe fn update_inv_partial(pid: i32, component: i32, slots: Vec<i32>, objs: Vec<i64>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateInvPartial::new(component, slots, objs));
    }

    return None;
}

#[wasm_bindgen(js_name = updateInvStopTransmit)]
pub unsafe fn update_inv_stop_transmit(pid: i32, component: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateInvStopTransmit::new(component));
    }

    return None;
}

#[wasm_bindgen(js_name = updatePid)]
pub unsafe fn update_pid(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdatePid::new(pid));
    }

    return None;
}

#[wasm_bindgen(js_name = updateRebootTimer)]
pub unsafe fn update_reboot_timer(pid: i32, ticks: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateRebootTimer::new(ticks));
    }

    return None;
}

#[wasm_bindgen(js_name = updateRunEnergy)]
pub unsafe fn update_runenergy(pid: i32, energy: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateRunEnergy::new(energy));
    }

    return None;
}

#[wasm_bindgen(js_name = updateRunWeight)]
pub unsafe fn update_runweight(pid: i32, kg: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateRunWeight::new(kg));
    }

    return None;
}

#[wasm_bindgen(js_name = updateStat)]
pub unsafe fn update_stat(pid: i32, stat: i32, experience: i32, level: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateStat::new(stat, experience, level));
    }

    return None;
}

#[wasm_bindgen(js_name = updateZoneFullFollows)]
pub unsafe fn update_zone_full_follows(pid: i32, x: i32, z: i32, originX: i32, originZ: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateZoneFullFollows::new(x, z, originX, originZ));
    }

    return None;
}

#[wasm_bindgen(js_name = updateZonePartialEnclosed)]
pub unsafe fn update_zone_partial_enclosed(pid: i32, x: i32, z: i32, originX: i32, originZ: i32, data: Vec<u8>) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateZonePartialEnclosed::new(x, z, originX, originZ, data));
    }

    return None;
}

#[wasm_bindgen(js_name = updateZonePartialFollows)]
pub unsafe fn update_zone_partial_follows(pid: i32, x: i32, z: i32, originX: i32, originZ: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write(&UpdateZonePartialFollows::new(x, z, originX, originZ));
    }

    return None;
}

#[wasm_bindgen(js_name = varp)]
pub unsafe fn varp(pid: i32, varp: i32, value: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return if value >= -128 && value <= 127 {
            player.write(&VarpSmall::new(varp, value))
        } else {
            player.write(&VarpLarge::new(varp, value))
        }
    }

    return None;
}

#[wasm_bindgen(js_name = nextBufferedWrite)]
pub unsafe fn next_buffered_write(pid: i32) -> Option<OutgoingPacket> {
    if pid == -1 {
        return None;
    }

    if let Some(Some(ref mut player)) = PLAYERS.get_mut(pid as usize) {
        return player.write_queue.pop_front();
    }

    return None;
}

// ---- decoders

static PACKET_LOOKUP: Lazy<HashMap<i32, Option<IncomingPacket>>> = Lazy::new(|| {
    use ClientInternalProt::*;
    use ClientProt::*;

    let mut lookup: HashMap<i32, Option<IncomingPacket>> = HashMap::new();
    lookup.insert(ClientInternalProt::CLIENT_CHEAT as i32, Some(IncomingPacket::new(ClientProt::CLIENT_CHEAT as i32, ClientCheat::length())));
    lookup.insert(ClientInternalProt::CLOSE_MODAL as i32, Some(IncomingPacket::new(ClientProt::CLOSE_MODAL as i32, CloseModal::length())));
    lookup.insert(ClientInternalProt::FRIENDLIST_ADD as i32, Some(IncomingPacket::new(ClientProt::FRIENDLIST_ADD as i32, FriendListAdd::length())));
    lookup.insert(ClientInternalProt::FRIENDLIST_DEL as i32, Some(IncomingPacket::new(ClientProt::FRIENDLIST_DEL as i32, FriendListDel::length())));
    lookup.insert(ClientInternalProt::IDLE_TIMER as i32, Some(IncomingPacket::new(ClientProt::IDLE_TIMER as i32, IdleTimer::length())));
    lookup.insert(ClientInternalProt::IF_BUTTON as i32, Some(IncomingPacket::new(ClientProt::IF_BUTTON as i32, IfButton::length())));
    lookup.insert(ClientInternalProt::IF_PLAYERDESIGN as i32, Some(IncomingPacket::new(ClientProt::IF_PLAYERDESIGN as i32, IfPlayerDesign::length())));
    lookup.insert(ClientInternalProt::IGNORELIST_ADD as i32, Some(IncomingPacket::new(ClientProt::IGNORELIST_ADD as i32, IgnoreListAdd::length())));
    lookup.insert(ClientInternalProt::IGNORELIST_DEL as i32, Some(IncomingPacket::new(ClientProt::IGNORELIST_DEL as i32, IgnoreListDel::length())));
    lookup.insert(ClientInternalProt::INV_BUTTON1 as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTON1 as i32, InvButton::length())));
    lookup.insert(ClientInternalProt::INV_BUTTON2 as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTON2 as i32, InvButton::length())));
    lookup.insert(ClientInternalProt::INV_BUTTON3 as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTON3 as i32, InvButton::length())));
    lookup.insert(ClientInternalProt::INV_BUTTON4 as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTON4 as i32, InvButton::length())));
    lookup.insert(ClientInternalProt::INV_BUTTON5 as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTON5 as i32, InvButton::length())));
    lookup.insert(ClientInternalProt::INV_BUTTOND as i32, Some(IncomingPacket::new(ClientProt::INV_BUTTOND as i32, InvButtonD::length())));
    lookup.insert(ClientInternalProt::MESSAGE_PRIVATE as i32, Some(IncomingPacket::new(ClientProt::MESSAGE_PRIVATE as i32, MessagePrivate::length())));
    lookup.insert(ClientInternalProt::MESSAGE_PUBLIC as i32, Some(IncomingPacket::new(ClientProt::MESSAGE_PUBLIC as i32, MessagePublic::length())));
    lookup.insert(ClientInternalProt::MOVE_MINIMAPCLICK as i32, Some(IncomingPacket::new(ClientProt::MOVE_MINIMAPCLICK as i32, MoveClick::length())));
    lookup.insert(ClientInternalProt::MOVE_GAMECLICK as i32, Some(IncomingPacket::new(ClientProt::MOVE_GAMECLICK as i32, MoveClick::length())));
    lookup.insert(ClientInternalProt::MOVE_OPCLICK as i32, Some(IncomingPacket::new(ClientProt::MOVE_OPCLICK as i32, MoveClick::length())));
    lookup.insert(ClientInternalProt::NO_TIMEOUT as i32, Some(IncomingPacket::new(ClientProt::NO_TIMEOUT as i32, NoTimeout::length())));
    lookup.insert(ClientInternalProt::OPHELD1 as i32, Some(IncomingPacket::new(ClientProt::OPHELD1 as i32, OpHeld::length())));
    lookup.insert(ClientInternalProt::OPHELD2 as i32, Some(IncomingPacket::new(ClientProt::OPHELD2 as i32, OpHeld::length())));
    lookup.insert(ClientInternalProt::OPHELD3 as i32, Some(IncomingPacket::new(ClientProt::OPHELD3 as i32, OpHeld::length())));
    lookup.insert(ClientInternalProt::OPHELD4 as i32, Some(IncomingPacket::new(ClientProt::OPHELD4 as i32, OpHeld::length())));
    lookup.insert(ClientInternalProt::OPHELD5 as i32, Some(IncomingPacket::new(ClientProt::OPHELD5 as i32, OpHeld::length())));
    lookup.insert(ClientInternalProt::OPHELDT as i32, Some(IncomingPacket::new(ClientProt::OPHELDT as i32, OpHeldT::length())));
    lookup.insert(ClientInternalProt::OPHELDU as i32, Some(IncomingPacket::new(ClientProt::OPHELDU as i32, OpHeldU::length())));
    lookup.insert(ClientInternalProt::OPLOC1 as i32, Some(IncomingPacket::new(ClientProt::OPLOC1 as i32, OpLoc::length())));
    lookup.insert(ClientInternalProt::OPLOC2 as i32, Some(IncomingPacket::new(ClientProt::OPLOC2 as i32, OpLoc::length())));
    lookup.insert(ClientInternalProt::OPLOC3 as i32, Some(IncomingPacket::new(ClientProt::OPLOC3 as i32, OpLoc::length())));
    lookup.insert(ClientInternalProt::OPLOC4 as i32, Some(IncomingPacket::new(ClientProt::OPLOC4 as i32, OpLoc::length())));
    lookup.insert(ClientInternalProt::OPLOC5 as i32, Some(IncomingPacket::new(ClientProt::OPLOC5 as i32, OpLoc::length())));
    lookup.insert(ClientInternalProt::OPLOCT as i32, Some(IncomingPacket::new(ClientProt::OPLOCT as i32, OpLocT::length())));
    lookup.insert(ClientInternalProt::OPLOCU as i32, Some(IncomingPacket::new(ClientProt::OPLOCU as i32, OpLocU::length())));
    lookup.insert(ClientInternalProt::OPNPC1 as i32, Some(IncomingPacket::new(ClientProt::OPNPC1 as i32, OpNpc::length())));
    lookup.insert(ClientInternalProt::OPNPC2 as i32, Some(IncomingPacket::new(ClientProt::OPNPC2 as i32, OpNpc::length())));
    lookup.insert(ClientInternalProt::OPNPC3 as i32, Some(IncomingPacket::new(ClientProt::OPNPC3 as i32, OpNpc::length())));
    lookup.insert(ClientInternalProt::OPNPC4 as i32, Some(IncomingPacket::new(ClientProt::OPNPC4 as i32, OpNpc::length())));
    lookup.insert(ClientInternalProt::OPNPC5 as i32, Some(IncomingPacket::new(ClientProt::OPNPC5 as i32, OpNpc::length())));
    lookup.insert(ClientInternalProt::OPNPCT as i32, Some(IncomingPacket::new(ClientProt::OPNPCT as i32, OpNpcT::length())));
    lookup.insert(ClientInternalProt::OPNPCU as i32, Some(IncomingPacket::new(ClientProt::OPNPCU as i32, OpNpcU::length())));
    lookup.insert(ClientInternalProt::OPOBJ1 as i32, Some(IncomingPacket::new(ClientProt::OPOBJ1 as i32, OpObj::length())));
    lookup.insert(ClientInternalProt::OPOBJ2 as i32, Some(IncomingPacket::new(ClientProt::OPOBJ2 as i32, OpObj::length())));
    lookup.insert(ClientInternalProt::OPOBJ3 as i32, Some(IncomingPacket::new(ClientProt::OPOBJ3 as i32, OpObj::length())));
    lookup.insert(ClientInternalProt::OPOBJ4 as i32, Some(IncomingPacket::new(ClientProt::OPOBJ4 as i32, OpObj::length())));
    lookup.insert(ClientInternalProt::OPOBJ5 as i32, Some(IncomingPacket::new(ClientProt::OPOBJ5 as i32, OpObj::length())));
    lookup.insert(ClientInternalProt::OPOBJT as i32, Some(IncomingPacket::new(ClientProt::OPOBJT as i32, OpObjT::length())));
    lookup.insert(ClientInternalProt::OPOBJU as i32, Some(IncomingPacket::new(ClientProt::OPOBJU as i32, OpObjU::length())));
    lookup.insert(ClientInternalProt::OPPLAYER1 as i32, Some(IncomingPacket::new(ClientProt::OPPLAYER1 as i32, OpPlayer::length())));
    lookup.insert(ClientInternalProt::OPPLAYER2 as i32, Some(IncomingPacket::new(ClientProt::OPPLAYER2 as i32, OpPlayer::length())));
    lookup.insert(ClientInternalProt::OPPLAYER3 as i32, Some(IncomingPacket::new(ClientProt::OPPLAYER3 as i32, OpPlayer::length())));
    lookup.insert(ClientInternalProt::OPPLAYER4 as i32, Some(IncomingPacket::new(ClientProt::OPPLAYER4 as i32, OpPlayer::length())));
    lookup.insert(ClientInternalProt::OPPLAYERT as i32, Some(IncomingPacket::new(ClientProt::OPPLAYERT as i32, OpPlayerT::length())));
    lookup.insert(ClientInternalProt::OPPLAYERU as i32, Some(IncomingPacket::new(ClientProt::OPPLAYERU as i32, OpPlayerU::length())));
    lookup.insert(ClientInternalProt::REBUILD_GETMAPS as i32, Some(IncomingPacket::new(ClientProt::REBUILD_GETMAPS as i32, RebuildGetMaps::length())));
    lookup.insert(ClientInternalProt::RESUME_PAUSEBUTTON as i32, Some(IncomingPacket::new(ClientProt::RESUME_PAUSEBUTTON as i32, ResumePauseButton::length())));
    lookup.insert(ClientInternalProt::RESUME_P_COUNTDIALOG as i32, Some(IncomingPacket::new(ClientProt::RESUME_P_COUNTDIALOG as i32, ResumePCountDialog::length())));
    lookup.insert(ClientInternalProt::TUTORIAL_CLICKSIDE as i32, Some(IncomingPacket::new(ClientProt::TUTORIAL_CLICKSIDE as i32, TutorialClickSide::length())));
    lookup.insert(ClientInternalProt::CHAT_SETMODE as i32, Some(IncomingPacket::new(ClientProt::CHAT_SETMODE as i32, ChatSetMode::length())));
    lookup.insert(ClientInternalProt::EVENT_TRACKING as i32, Some(IncomingPacket::new(ClientProt::EVENT_TRACKING as i32, EventTracking::length())));
    lookup.insert(ClientInternalProt::REPORT_ABUSE as i32, Some(IncomingPacket::new(ClientProt::REPORT_ABUSE as i32, ReportAbuse::length())));
    lookup.insert(ClientInternalProt::EVENT_CAMERA_POSITION as i32, Some(IncomingPacket::new(ClientProt::EVENT_CAMERA_POSITION as i32, EventCameraPosition::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC1 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC1 as i32, AnticheatOp1::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC2 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC2 as i32, AnticheatOp2::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC3 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC3 as i32, AnticheatOp3::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC4 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC4 as i32, AnticheatOp4::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC5 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC5 as i32, AnticheatOp5::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC6 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC6 as i32, AnticheatOp6::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC7 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC7 as i32, AnticheatOp7::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC8 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC8 as i32, AnticheatOp8::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_OPLOGIC9 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_OPLOGIC9 as i32, AnticheatOp9::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC1 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC1 as i32, AnticheatCycle1::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC2 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC2 as i32, AnticheatCycle2::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC3 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC3 as i32, AnticheatCycle3::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC4 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC4 as i32, AnticheatCycle4::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC5 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC5 as i32, AnticheatCycle5::length())));
    lookup.insert(ClientInternalProt::ANTICHEAT_CYCLELOGIC6 as i32, Some(IncomingPacket::new(ClientProt::ANTICHEAT_CYCLELOGIC6 as i32, AnticheatCycle6::length())));
    return lookup;
});

#[wasm_bindgen(js_name = nextBufferedRead)]
pub unsafe fn next_buffered_read(id: i32) -> Option<IncomingPacket> {
    return match PACKET_LOOKUP.get(&id) {
        None => None,
        Some(packet) => packet.clone(),
    }
}

unsafe fn read(bytes: Vec<u8>) -> Packet {
    let mut buf: Packet = Packet::new(bytes.len());
    buf.pdata(&bytes, 0, bytes.len());
    buf.pos = 0;
    return buf;
}

#[wasm_bindgen(js_name = clientCheat)]
pub unsafe fn clientCheat(bytes: Vec<u8>) -> Option<ClientCheat> {
    return Some(ClientCheat::decode(ClientProt::CLIENT_CHEAT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = closeModal)]
pub unsafe fn close_modal(bytes: Vec<u8>) -> Option<CloseModal> {
    return Some(CloseModal::decode(ClientProt::CLOSE_MODAL, &mut read(bytes)));
}

#[wasm_bindgen(js_name = friendListAdd)]
pub unsafe fn friend_list_add(bytes: Vec<u8>) -> Option<FriendListAdd> {
    return Some(FriendListAdd::decode(ClientProt::FRIENDLIST_ADD, &mut read(bytes)));
}

#[wasm_bindgen(js_name = friendListDel)]
pub unsafe fn friend_list_del(bytes: Vec<u8>) -> Option<FriendListDel> {
    return Some(FriendListDel::decode(ClientProt::FRIENDLIST_DEL, &mut read(bytes)));
}

#[wasm_bindgen(js_name = idleTimer)]
pub unsafe fn idle_timer(bytes: Vec<u8>) -> Option<IdleTimer> {
    return Some(IdleTimer::decode(ClientProt::IDLE_TIMER, &mut read(bytes)));
}

#[wasm_bindgen(js_name = ifButton)]
pub unsafe fn if_button(bytes: Vec<u8>) -> Option<IfButton> {
    return Some(IfButton::decode(ClientProt::IF_BUTTON, &mut read(bytes)));
}

#[wasm_bindgen(js_name = ifPlayerDesign)]
pub unsafe fn if_player_design(bytes: Vec<u8>) -> Option<IfPlayerDesign> {
    return Some(IfPlayerDesign::decode(ClientProt::IF_PLAYERDESIGN, &mut read(bytes)));
}

#[wasm_bindgen(js_name = ignoreListAdd)]
pub unsafe fn ignore_list_add(bytes: Vec<u8>) -> Option<IgnoreListAdd> {
    return Some(IgnoreListAdd::decode(ClientProt::IGNORELIST_ADD, &mut read(bytes)));
}

#[wasm_bindgen(js_name = ignoreListDel)]
pub unsafe fn ignore_list_del(bytes: Vec<u8>) -> Option<IgnoreListDel> {
    return Some(IgnoreListDel::decode(ClientProt::IGNORELIST_DEL, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButton1)]
pub unsafe fn inv_button1(bytes: Vec<u8>) -> Option<InvButton> {
    return Some(InvButton::decode(ClientProt::INV_BUTTON1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButton2)]
pub unsafe fn inv_button2(bytes: Vec<u8>) -> Option<InvButton> {
    return Some(InvButton::decode(ClientProt::INV_BUTTON2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButton3)]
pub unsafe fn inv_button3(bytes: Vec<u8>) -> Option<InvButton> {
    return Some(InvButton::decode(ClientProt::INV_BUTTON3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButton4)]
pub unsafe fn inv_button4(bytes: Vec<u8>) -> Option<InvButton> {
    return Some(InvButton::decode(ClientProt::INV_BUTTON4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButton5)]
pub unsafe fn inv_button5(bytes: Vec<u8>) -> Option<InvButton> {
    return Some(InvButton::decode(ClientProt::INV_BUTTON5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = invButtonD)]
pub unsafe fn inv_buttonD(bytes: Vec<u8>) -> Option<InvButtonD> {
    return Some(InvButtonD::decode(ClientProt::INV_BUTTOND, &mut read(bytes)));
}

#[wasm_bindgen(js_name = messagePrivate)]
pub unsafe fn message_private(bytes: Vec<u8>) -> Option<MessagePrivate> {
    return Some(MessagePrivate::decode(ClientProt::MESSAGE_PRIVATE, &mut read(bytes)));
}

#[wasm_bindgen(js_name = messagePublic)]
pub unsafe fn message_public(bytes: Vec<u8>) -> Option<MessagePublic> {
    return Some(MessagePublic::decode(ClientProt::MESSAGE_PUBLIC, &mut read(bytes)));
}

#[wasm_bindgen(js_name = moveMinimapClick)]
pub unsafe fn move_minimap_click(bytes: Vec<u8>) -> Option<MoveClick> {
    return Some(MoveClick::decode(ClientProt::MOVE_MINIMAPCLICK, &mut read(bytes)));
}

#[wasm_bindgen(js_name = moveGameClick)]
pub unsafe fn move_game_click(bytes: Vec<u8>) -> Option<MoveClick> {
    return Some(MoveClick::decode(ClientProt::MOVE_GAMECLICK, &mut read(bytes)));
}

#[wasm_bindgen(js_name = moveOpClick)]
pub unsafe fn move_op_click(bytes: Vec<u8>) -> Option<MoveClick> {
    return Some(MoveClick::decode(ClientProt::MOVE_OPCLICK, &mut read(bytes)));
}

#[wasm_bindgen(js_name = noTimeout)]
pub unsafe fn no_timeout(bytes: Vec<u8>) -> Option<NoTimeout> {
    return Some(NoTimeout::decode(ClientProt::NO_TIMEOUT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheld1)]
pub unsafe fn opheld1(bytes: Vec<u8>) -> Option<OpHeld> {
    return Some(OpHeld::decode(ClientProt::OPHELD1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheld2)]
pub unsafe fn opheld2(bytes: Vec<u8>) -> Option<OpHeld> {
    return Some(OpHeld::decode(ClientProt::OPHELD2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheld3)]
pub unsafe fn opheld3(bytes: Vec<u8>) -> Option<OpHeld> {
    return Some(OpHeld::decode(ClientProt::OPHELD3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheld4)]
pub unsafe fn opheld4(bytes: Vec<u8>) -> Option<OpHeld> {
    return Some(OpHeld::decode(ClientProt::OPHELD4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheld5)]
pub unsafe fn opheld5(bytes: Vec<u8>) -> Option<OpHeld> {
    return Some(OpHeld::decode(ClientProt::OPHELD5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheldT)]
pub unsafe fn opheldT(bytes: Vec<u8>) -> Option<OpHeldT> {
    return Some(OpHeldT::decode(ClientProt::OPHELDT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opheldU)]
pub unsafe fn opheldU(bytes: Vec<u8>) -> Option<OpHeldU> {
    return Some(OpHeldU::decode(ClientProt::OPHELDU, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oploc1)]
pub unsafe fn oploc1(bytes: Vec<u8>) -> Option<OpLoc> {
    return Some(OpLoc::decode(ClientProt::OPLOC1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oploc2)]
pub unsafe fn oploc2(bytes: Vec<u8>) -> Option<OpLoc> {
    return Some(OpLoc::decode(ClientProt::OPLOC2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oploc3)]
pub unsafe fn oploc3(bytes: Vec<u8>) -> Option<OpLoc> {
    return Some(OpLoc::decode(ClientProt::OPLOC3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oploc4)]
pub unsafe fn oploc4(bytes: Vec<u8>) -> Option<OpLoc> {
    return Some(OpLoc::decode(ClientProt::OPLOC4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oploc5)]
pub unsafe fn oploc5(bytes: Vec<u8>) -> Option<OpLoc> {
    return Some(OpLoc::decode(ClientProt::OPLOC5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oplocT)]
pub unsafe fn oplocT(bytes: Vec<u8>) -> Option<OpLocT> {
    return Some(OpLocT::decode(ClientProt::OPLOCT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = oplocU)]
pub unsafe fn oplocU(bytes: Vec<u8>) -> Option<OpLocU> {
    return Some(OpLocU::decode(ClientProt::OPLOCU, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpc1)]
pub unsafe fn opnpc1(bytes: Vec<u8>) -> Option<OpNpc> {
    return Some(OpNpc::decode(ClientProt::OPNPC1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpc2)]
pub unsafe fn opnpc2(bytes: Vec<u8>) -> Option<OpNpc> {
    return Some(OpNpc::decode(ClientProt::OPNPC2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpc3)]
pub unsafe fn opnpc3(bytes: Vec<u8>) -> Option<OpNpc> {
    return Some(OpNpc::decode(ClientProt::OPNPC3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpc4)]
pub unsafe fn opnpc4(bytes: Vec<u8>) -> Option<OpNpc> {
    return Some(OpNpc::decode(ClientProt::OPNPC4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpc5)]
pub unsafe fn opnpc5(bytes: Vec<u8>) -> Option<OpNpc> {
    return Some(OpNpc::decode(ClientProt::OPNPC5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpcT)]
pub unsafe fn opnpcT(bytes: Vec<u8>) -> Option<OpNpcT> {
    return Some(OpNpcT::decode(ClientProt::OPNPCT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opnpcU)]
pub unsafe fn opnpcU(bytes: Vec<u8>) -> Option<OpNpcU> {
    return Some(OpNpcU::decode(ClientProt::OPNPCU, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobj1)]
pub unsafe fn opobj1(bytes: Vec<u8>) -> Option<OpObj> {
    return Some(OpObj::decode(ClientProt::OPOBJ1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobj2)]
pub unsafe fn opobj2(bytes: Vec<u8>) -> Option<OpObj> {
    return Some(OpObj::decode(ClientProt::OPOBJ2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobj3)]
pub unsafe fn opobj3(bytes: Vec<u8>) -> Option<OpObj> {
    return Some(OpObj::decode(ClientProt::OPOBJ3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobj4)]
pub unsafe fn opobj4(bytes: Vec<u8>) -> Option<OpObj> {
    return Some(OpObj::decode(ClientProt::OPOBJ4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobj5)]
pub unsafe fn opobj5(bytes: Vec<u8>) -> Option<OpObj> {
    return Some(OpObj::decode(ClientProt::OPOBJ5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobjT)]
pub unsafe fn opobjT(bytes: Vec<u8>) -> Option<OpObjT> {
    return Some(OpObjT::decode(ClientProt::OPOBJT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opobjU)]
pub unsafe fn opobjU(bytes: Vec<u8>) -> Option<OpObjU> {
    return Some(OpObjU::decode(ClientProt::OPOBJU, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayer1)]
pub unsafe fn opplayer1(bytes: Vec<u8>) -> Option<OpPlayer> {
    return Some(OpPlayer::decode(ClientProt::OPPLAYER1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayer2)]
pub unsafe fn opplayer2(bytes: Vec<u8>) -> Option<OpPlayer> {
    return Some(OpPlayer::decode(ClientProt::OPPLAYER2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayer3)]
pub unsafe fn opplayer3(bytes: Vec<u8>) -> Option<OpPlayer> {
    return Some(OpPlayer::decode(ClientProt::OPPLAYER3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayer4)]
pub unsafe fn opplayer4(bytes: Vec<u8>) -> Option<OpPlayer> {
    return Some(OpPlayer::decode(ClientProt::OPPLAYER4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayerT)]
pub unsafe fn opplayerT(bytes: Vec<u8>) -> Option<OpPlayerT> {
    return Some(OpPlayerT::decode(ClientProt::OPPLAYERT, &mut read(bytes)));
}

#[wasm_bindgen(js_name = opplayerU)]
pub unsafe fn opplayerU(bytes: Vec<u8>) -> Option<OpPlayerU> {
    return Some(OpPlayerU::decode(ClientProt::OPPLAYERU, &mut read(bytes)));
}

#[wasm_bindgen(js_name = rebuildGetMaps)]
pub unsafe fn rebuild_getmaps(bytes: Vec<u8>) -> Option<RebuildGetMaps> {
    return Some(RebuildGetMaps::decode(ClientProt::REBUILD_GETMAPS, &mut read(bytes)));
}

#[wasm_bindgen(js_name = resumePauseButton)]
pub unsafe fn resume_pausebutton(bytes: Vec<u8>) -> Option<ResumePauseButton> {
    return Some(ResumePauseButton::decode(ClientProt::RESUME_PAUSEBUTTON, &mut read(bytes)));
}

#[wasm_bindgen(js_name = resumeCountDialog)]
pub unsafe fn resume_countdialog(bytes: Vec<u8>) -> Option<ResumePCountDialog> {
    return Some(ResumePCountDialog::decode(ClientProt::RESUME_P_COUNTDIALOG, &mut read(bytes)));
}

#[wasm_bindgen(js_name = tutorialClickSide)]
pub unsafe fn tutorial_clickside(bytes: Vec<u8>) -> Option<TutorialClickSide> {
    return Some(TutorialClickSide::decode(ClientProt::TUTORIAL_CLICKSIDE, &mut read(bytes)));
}

#[wasm_bindgen(js_name = chatSetMode)]
pub unsafe fn chat_setmode(bytes: Vec<u8>) -> Option<ChatSetMode> {
    return Some(ChatSetMode::decode(ClientProt::CHAT_SETMODE, &mut read(bytes)));
}

#[wasm_bindgen(js_name = eventTracking)]
pub unsafe fn event_tracking(bytes: Vec<u8>) -> Option<EventTracking> {
    return Some(EventTracking::decode(ClientProt::EVENT_TRACKING, &mut read(bytes)));
}

#[wasm_bindgen(js_name = reportAbuse)]
pub unsafe fn report_abuse(bytes: Vec<u8>) -> Option<ReportAbuse> {
    return Some(ReportAbuse::decode(ClientProt::REPORT_ABUSE, &mut read(bytes)));
}

#[wasm_bindgen(js_name = eventCameraPosition)]
pub unsafe fn event_camera_position(bytes: Vec<u8>) -> Option<EventCameraPosition> {
    return Some(EventCameraPosition::decode(ClientProt::EVENT_CAMERA_POSITION, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp1)]
pub unsafe fn anticheatop1(bytes: Vec<u8>) -> Option<AnticheatOp1> {
    return Some(AnticheatOp1::decode(ClientProt::ANTICHEAT_OPLOGIC1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp2)]
pub unsafe fn anticheatop2(bytes: Vec<u8>) -> Option<AnticheatOp2> {
    return Some(AnticheatOp2::decode(ClientProt::ANTICHEAT_OPLOGIC2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp3)]
pub unsafe fn anticheatop3(bytes: Vec<u8>) -> Option<AnticheatOp3> {
    return Some(AnticheatOp3::decode(ClientProt::ANTICHEAT_OPLOGIC3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp4)]
pub unsafe fn anticheatop4(bytes: Vec<u8>) -> Option<AnticheatOp4> {
    return Some(AnticheatOp4::decode(ClientProt::ANTICHEAT_OPLOGIC4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp5)]
pub unsafe fn anticheatop5(bytes: Vec<u8>) -> Option<AnticheatOp5> {
    return Some(AnticheatOp5::decode(ClientProt::ANTICHEAT_OPLOGIC5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp6)]
pub unsafe fn anticheatop6(bytes: Vec<u8>) -> Option<AnticheatOp6> {
    return Some(AnticheatOp6::decode(ClientProt::ANTICHEAT_OPLOGIC6, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp7)]
pub unsafe fn anticheatop7(bytes: Vec<u8>) -> Option<AnticheatOp7> {
    return Some(AnticheatOp7::decode(ClientProt::ANTICHEAT_OPLOGIC7, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp8)]
pub unsafe fn anticheatop8(bytes: Vec<u8>) -> Option<AnticheatOp8> {
    return Some(AnticheatOp8::decode(ClientProt::ANTICHEAT_OPLOGIC8, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatOp9)]
pub unsafe fn anticheatop9(bytes: Vec<u8>) -> Option<AnticheatOp9> {
    return Some(AnticheatOp9::decode(ClientProt::ANTICHEAT_OPLOGIC9, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle1)]
pub unsafe fn anticheatcycle1(bytes: Vec<u8>) -> Option<AnticheatCycle1> {
    return Some(AnticheatCycle1::decode(ClientProt::ANTICHEAT_CYCLELOGIC1, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle2)]
pub unsafe fn anticheatcycle2(bytes: Vec<u8>) -> Option<AnticheatCycle2> {
    return Some(AnticheatCycle2::decode(ClientProt::ANTICHEAT_CYCLELOGIC2, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle3)]
pub unsafe fn anticheatcycle3(bytes: Vec<u8>) -> Option<AnticheatCycle3> {
    return Some(AnticheatCycle3::decode(ClientProt::ANTICHEAT_CYCLELOGIC3, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle4)]
pub unsafe fn anticheatcycle4(bytes: Vec<u8>) -> Option<AnticheatCycle4> {
    return Some(AnticheatCycle4::decode(ClientProt::ANTICHEAT_CYCLELOGIC4, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle5)]
pub unsafe fn anticheatcycle5(bytes: Vec<u8>) -> Option<AnticheatCycle5> {
    return Some(AnticheatCycle5::decode(ClientProt::ANTICHEAT_CYCLELOGIC5, &mut read(bytes)));
}

#[wasm_bindgen(js_name = anticheatCycle6)]
pub unsafe fn anticheatcycle6(bytes: Vec<u8>) -> Option<AnticheatCycle6> {
    return Some(AnticheatCycle6::decode(ClientProt::ANTICHEAT_CYCLELOGIC6, &mut read(bytes)));
}

// ---- misc

#[wasm_bindgen(js_name = unpackWords)]
pub unsafe fn unpack_words(bytes: Vec<u8>, length: usize) -> String {
    return WordPack::unpack(&mut Packet::from(bytes), length);
}

#[wasm_bindgen(js_name = packWords)]
pub unsafe fn pack_words(bytes: Vec<u8>, msg: String) -> Vec<u8> {
    let mut buf: Packet = Packet::from(bytes);
    WordPack::pack(&mut buf, msg);
    return buf.data;
}
