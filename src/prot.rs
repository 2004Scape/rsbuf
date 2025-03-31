#![allow(non_camel_case_types)]
#![allow(dead_code)]

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(u16)]
#[derive(PartialEq)]
#[wasm_bindgen]
pub enum PlayerInfoProt {
    APPEARANCE = 0x1,
    ANIM = 0x2,
    FACE_ENTITY = 0x4,
    SAY = 0x8,
    DAMAGE = 0x10,
    FACE_COORD = 0x20,
    CHAT = 0x40,
    BIG = 0x80,
    SPOT_ANIM = 0x100,
    EXACT_MOVE = 0x200,
}

impl PlayerInfoProt {
    #[inline]
    pub const fn to_index(self) -> usize {
        // the ordering here does not matter.
        return match self {
            PlayerInfoProt::APPEARANCE => 0,
            PlayerInfoProt::ANIM => 1,
            PlayerInfoProt::FACE_ENTITY => 2,
            PlayerInfoProt::SAY => 3,
            PlayerInfoProt::DAMAGE => 4,
            PlayerInfoProt::FACE_COORD => 5,
            PlayerInfoProt::CHAT => 6,
            PlayerInfoProt::SPOT_ANIM => 7,
            PlayerInfoProt::BIG => 255, // unused
            PlayerInfoProt::EXACT_MOVE => 255, // unused
        }
    }
}

#[repr(u16)]
#[wasm_bindgen]
pub enum NpcInfoProt {
    ANIM = 0x2,
    FACE_ENTITY = 0x4,
    SAY = 0x8,
    DAMAGE = 0x10,
    CHANGE_TYPE = 0x20,
    SPOT_ANIM = 0x40,
    FACE_COORD = 0x80,
}

impl NpcInfoProt {
    #[inline]
    pub const fn to_index(self) -> usize {
        // the ordering here does not matter.
        return match self {
            NpcInfoProt::ANIM => 0,
            NpcInfoProt::FACE_ENTITY => 1,
            NpcInfoProt::SAY => 2,
            NpcInfoProt::DAMAGE => 3,
            NpcInfoProt::CHANGE_TYPE => 4,
            NpcInfoProt::SPOT_ANIM => 5,
            NpcInfoProt::FACE_COORD => 6,
        }
    }
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum ClientProt {
    REBUILD_GETMAPS,

    NO_TIMEOUT,

    IDLE_TIMER,

    EVENT_TRACKING,
    EVENT_CAMERA_POSITION,

    ANTICHEAT_OPLOGIC1,
    ANTICHEAT_OPLOGIC2,
    ANTICHEAT_OPLOGIC3,
    ANTICHEAT_OPLOGIC4,
    ANTICHEAT_OPLOGIC5,
    ANTICHEAT_OPLOGIC6,
    ANTICHEAT_OPLOGIC7,
    ANTICHEAT_OPLOGIC8,
    ANTICHEAT_OPLOGIC9,

    ANTICHEAT_CYCLELOGIC1,
    ANTICHEAT_CYCLELOGIC2,
    ANTICHEAT_CYCLELOGIC3,
    ANTICHEAT_CYCLELOGIC4,
    ANTICHEAT_CYCLELOGIC5,
    ANTICHEAT_CYCLELOGIC6,

    OPOBJ1,
    OPOBJ2,
    OPOBJ3,
    OPOBJ4,
    OPOBJ5,
    OPOBJT,
    OPOBJU,

    OPNPC1,
    OPNPC2,
    OPNPC3,
    OPNPC4,
    OPNPC5,
    OPNPCT,
    OPNPCU,

    OPLOC1,
    OPLOC2,
    OPLOC3,
    OPLOC4,
    OPLOC5,
    OPLOCT,
    OPLOCU,

    OPPLAYER1,
    OPPLAYER2,
    OPPLAYER3,
    OPPLAYER4,
    OPPLAYERT,
    OPPLAYERU,

    OPHELD1,
    OPHELD2,
    OPHELD3,
    OPHELD4,
    OPHELD5,
    OPHELDT,
    OPHELDU,

    INV_BUTTON1,
    INV_BUTTON2,
    INV_BUTTON3,
    INV_BUTTON4,
    INV_BUTTON5,
    IF_BUTTON,

    RESUME_PAUSEBUTTON,
    CLOSE_MODAL,
    RESUME_P_COUNTDIALOG,
    TUTORIAL_CLICKSIDE,

    MOVE_OPCLICK,
    REPORT_ABUSE,
    MOVE_MINIMAPCLICK,
    INV_BUTTOND,
    IGNORELIST_DEL,
    IGNORELIST_ADD,
    IF_PLAYERDESIGN,
    CHAT_SETMODE,
    MESSAGE_PRIVATE,
    FRIENDLIST_DEL,
    FRIENDLIST_ADD,
    CLIENT_CHEAT,
    MESSAGE_PUBLIC,
    MOVE_GAMECLICK,
}

#[repr(u8)]
pub enum ServerInternalProt {
    // interfaces
    IF_OPENCHAT = 14,
    IF_OPENMAIN_SIDE = 28,
    IF_CLOSE = 129,
    IF_SETTAB = 167,
    IF_OPENMAIN = 168,
    IF_OPENSIDE = 195,

    // updating interfaces
    IF_SETCOLOUR = 2, // NXT naming
    IF_SETHIDE = 26, // NXT naming
    IF_SETOBJECT = 46, // NXT naming
    IF_SETTAB_ACTIVE = 84,
    IF_SETMODEL = 87, // NXT naming
    IF_SETRECOL = 103, // NXT naming
    IF_SETANIM = 146, // NXT naming
    IF_SETPLAYERHEAD = 197, // NXT naming
    IF_SETTEXT = 201, // NXT naming
    IF_SETNPCHEAD = 204, // NXT naming
    IF_SETPOSITION = 209, // NXT naming

    // tutorial area
    TUT_FLASH = 126,
    TUT_OPEN = 185,

    // inventory
    UPDATE_INV_STOP_TRANSMIT = 15, // NXT naming
    UPDATE_INV_FULL = 98, // NXT naming
    UPDATE_INV_PARTIAL = 213, // NXT naming

    // camera control
    CAM_LOOKAT = 74, // NXT naming
    CAM_SHAKE = 13, // NXT naming
    CAM_MOVETO = 3, // NXT naming
    CAM_RESET = 239, // NXT naming

    // entity updates
    NPC_INFO = 1, // NXT naming
    PLAYER_INFO = 184, // NXT naming

    // input tracking
    FINISH_TRACKING = 133,
    ENABLE_TRACKING = 226,

    // social
    MESSAGE_GAME = 4, // NXT naming
    UPDATE_IGNORELIST = 21, // NXT naming
    CHAT_FILTER_SETTINGS = 32, // NXT naming
    MESSAGE_PRIVATE = 41, // NXT naming
    UPDATE_FRIENDLIST = 152, // NXT naming

    // misc
    UNSET_MAP_FLAG = 19, // NXT has "SET_MAP_FLAG" but we cannot control the position
    UPDATE_RUNWEIGHT = 22, // NXT naming
    HINT_ARROW = 25, // NXT naming
    UPDATE_REBOOT_TIMER = 43, // NXT naming
    UPDATE_STAT = 44, // NXT naming
    UPDATE_RUNENERGY = 68, // NXT naming
    RESET_ANIMS = 136, // NXT naming
    UPDATE_PID = 139,
    LAST_LOGIN_INFO = 140, // NXT naming
    LOGOUT = 142, // NXT naming
    P_COUNTDIALOG = 243, // named after runescript command + client resume_p_countdialog packet
    SET_MULTIWAY = 254,

    // maps
    DATA_LOC_DONE = 20,
    DATA_LAND_DONE = 80,
    DATA_LAND = 132,
    DATA_LOC = 220,
    REBUILD_NORMAL = 237, // NXT naming (do we really need _normal if there's no region rebuild?)

    // vars
    VARP_SMALL = 150, // NXT naming
    VARP_LARGE = 175, // NXT naming
    RESET_CLIENT_VARCACHE = 193, // NXT naming

    // audio
    SYNTH_SOUND = 12, // NXT naming
    MIDI_SONG = 54, // NXT naming
    MIDI_JINGLE = 212, // NXT naming

    // zones
    UPDATE_ZONE_PARTIAL_FOLLOWS = 7, // NXT naming
    UPDATE_ZONE_FULL_FOLLOWS = 135, // NXT naming
    UPDATE_ZONE_PARTIAL_ENCLOSED = 162, // NXT naming
    LOC_MERGE = 23, // based on runescript command p_locmerge
    LOC_ANIM = 42, // NXT naming
    OBJ_DEL = 49, // NXT naming
    OBJ_REVEAL = 50, // NXT naming
    LOC_ADD_CHANGE = 59, // NXT naming
    MAP_PROJANIM = 69, // NXT naming
    LOC_DEL = 76, // NXT naming
    OBJ_COUNT = 151, // NXT naming
    MAP_ANIM = 191, // NXT naming
    OBJ_ADD = 223, // NXT naming
}

#[repr(u8)]
pub enum ClientInternalProt {
    REBUILD_GETMAPS = 150,

    NO_TIMEOUT = 108, // NXT naming

    IDLE_TIMER = 70,

    EVENT_TRACKING = 81,
    EVENT_CAMERA_POSITION = 189, // NXT naming

    // autogenerated as part of obfuscation process
    ANTICHEAT_OPLOGIC1 = 7,
    ANTICHEAT_OPLOGIC2 = 88,
    ANTICHEAT_OPLOGIC3 = 30,
    ANTICHEAT_OPLOGIC4 = 176,
    ANTICHEAT_OPLOGIC5 = 220,
    ANTICHEAT_OPLOGIC6 = 66,
    ANTICHEAT_OPLOGIC7 = 17,
    ANTICHEAT_OPLOGIC8 = 2,
    ANTICHEAT_OPLOGIC9 = 238,

    // autogenerated as part of obfuscation process
    ANTICHEAT_CYCLELOGIC1 = 233,
    ANTICHEAT_CYCLELOGIC2 = 146,
    ANTICHEAT_CYCLELOGIC3 = 215,
    ANTICHEAT_CYCLELOGIC4 = 236,
    ANTICHEAT_CYCLELOGIC5 = 85,
    ANTICHEAT_CYCLELOGIC6 = 219,

    OPOBJ1 = 140, // NXT naming
    OPOBJ2 = 40, // NXT naming
    OPOBJ3 = 200, // NXT naming
    OPOBJ4 = 178, // NXT naming
    OPOBJ5 = 247, // NXT naming
    OPOBJT = 138, // NXT naming
    OPOBJU = 239, // NXT naming

    OPNPC1 = 194, // NXT naming
    OPNPC2 = 8, // NXT naming
    OPNPC3 = 27, // NXT naming
    OPNPC4 = 113, // NXT naming
    OPNPC5 = 100, // NXT naming
    OPNPCT = 134, // NXT naming
    OPNPCU = 202, // NXT naming

    OPLOC1 = 245, // NXT naming
    OPLOC2 = 172, // NXT naming
    OPLOC3 = 96, // NXT naming
    OPLOC4 = 97, // NXT naming
    OPLOC5 = 116, // NXT naming
    OPLOCT = 9, // NXT naming
    OPLOCU = 75, // NXT naming

    OPPLAYER1 = 164, // NXT naming
    OPPLAYER2 = 53, // NXT naming
    OPPLAYER3 = 185, // NXT naming
    OPPLAYER4 = 206, // NXT naming
    OPPLAYERT = 177, // NXT naming
    OPPLAYERU = 248, // NXT naming

    OPHELD1 = 195, // name based on runescript trigger
    OPHELD2 = 71, // name based on runescript trigger
    OPHELD3 = 133, // name based on runescript trigger
    OPHELD4 = 157, // name based on runescript trigger
    OPHELD5 = 211, // name based on runescript trigger
    OPHELDT = 48, // name based on runescript trigger
    OPHELDU = 130, // name based on runescript trigger

    INV_BUTTON1 = 31, // NXT has "IF_BUTTON1" but for our interface system, this makes more sense
    INV_BUTTON2 = 59, // NXT has "IF_BUTTON2" but for our interface system, this makes more sense
    INV_BUTTON3 = 212, // NXT has "IF_BUTTON3" but for our interface system, this makes more sense
    INV_BUTTON4 = 38, // NXT has "IF_BUTTON4" but for our interface system, this makes more sense
    INV_BUTTON5 = 6, // NXT has "IF_BUTTON5" but for our interface system, this makes more sense
    IF_BUTTON = 155, // NXT naming

    RESUME_PAUSEBUTTON = 235, // NXT naming
    CLOSE_MODAL = 231, // NXT naming
    RESUME_P_COUNTDIALOG = 237, // NXT naming
    TUTORIAL_CLICKSIDE = 175,

    MOVE_OPCLICK = 93, // comes with OP packets, name based on other MOVE packets
    REPORT_ABUSE = 190,
    MOVE_MINIMAPCLICK = 165, // NXT naming
    INV_BUTTOND = 159, // NXT has "IF_BUTTOND" but for our interface system, this makes more sense
    IGNORELIST_DEL = 171, // NXT naming
    IGNORELIST_ADD = 79, // NXT naming
    IF_PLAYERDESIGN = 52,
    CHAT_SETMODE = 244, // NXT naming
    MESSAGE_PRIVATE = 148, // NXT naming
    FRIENDLIST_DEL = 11, // NXT naming
    FRIENDLIST_ADD = 118, // NXT naming
    CLIENT_CHEAT = 4, // NXT naming
    MESSAGE_PUBLIC = 158, // NXT naming
    MOVE_GAMECLICK = 181, // NXT naming
}