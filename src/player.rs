use crate::build::BuildArea;
use crate::coord::CoordGrid;
use crate::visibility::Visibility;

#[derive(Clone)]
pub struct Player {
    pub coord: CoordGrid,
    pub origin: CoordGrid,
    pub pid: i32,
    pub tele: bool,
    pub jump: bool,
    pub run_dir: i8,
    pub walk_dir: i8,
    pub visibility: Visibility,
    pub active: bool,
    pub build: BuildArea,
    pub masks: u32,
    pub appearance: Vec<u8>,
    pub last_appearance: i32,
    pub face_entity: i32,
    pub face_x: i32,
    pub face_z: i32,
    pub orientation_x: i32,
    pub orientation_z: i32,
    pub damage_taken: i32,
    pub damage_type: i32,
    pub damage_taken2: i32,
    pub damage_type2: i32,
    pub current_hitpoints: i32,
    pub base_hitpoints: i32,
    pub anim_id: i32,
    pub anim_delay: i32,
    pub say: Option<String>,
    pub chat: Option<Chat>,
    pub graphic_id: i32,
    pub graphic_height: i32,
    pub graphic_delay: i32,
    pub exact_move: Option<ExactMove>,
}

#[derive(Clone)]
pub struct Chat {
    pub bytes: Vec<u8>,
    pub color: u8,
    pub effect: u8,
    pub ignored: u8,
}

#[derive(Clone)]
pub struct ExactMove {
    pub start_x: i32,
    pub start_z: i32,
    pub end_x: i32,
    pub end_z: i32,
    pub begin: i32,
    pub finish: i32,
    pub dir: i32,
}

impl Player {
    #[inline]
    pub fn new(pid: i32) -> Player {
        return Player {
            coord: CoordGrid::from(0, 0, 0),
            origin: CoordGrid::from(0, 0, 0),
            pid,
            tele: false,
            jump: false,
            run_dir: -1,
            walk_dir: -1,
            visibility: Visibility::DEFAULT,
            active: false,
            build: BuildArea::new(),
            masks: 0,
            appearance: vec![],
            last_appearance: -1,
            face_entity: -1,
            face_x: -1,
            face_z: -1,
            orientation_x: -1,
            orientation_z: -1,
            damage_taken: -1,
            damage_type: -1,
            damage_taken2: -1,
            damage_type2: -1,
            current_hitpoints: -1,
            base_hitpoints: -1,
            anim_id: -1,
            anim_delay: -1,
            say: None,
            chat: None,
            graphic_id: -1,
            graphic_height: -1,
            graphic_delay: -1,
            exact_move: None,
        }
    }

    #[inline]
    pub fn cleanup(&mut self) {
        self.walk_dir = -1;
        self.run_dir = -1;
        self.jump = false;
        self.tele = false;
        self.masks = 0;
        // self.appearance = vec![];
        // self.last_appearance = -1;
        // self.face_entity = -1;
        self.face_x = -1;
        self.face_z = -1;
        // self.orientation_x = -1;
        // self.orientation_z = -1;
        self.damage_taken = -1;
        self.damage_type = -1;
        self.damage_taken2 = -1;
        self.damage_type2 = -1;
        self.current_hitpoints = -1;
        self.base_hitpoints = -1;
        self.anim_id = -1;
        self.anim_delay = -1;
        self.say = None;
        self.chat = None;
        self.graphic_id = -1;
        self.graphic_height = -1;
        self.graphic_delay = -1;
        self.exact_move = None;
    }
}

impl ExactMove {
    #[inline]
    pub const fn new(
        start_x: i32,
        start_z: i32,
        end_x: i32,
        end_z: i32,
        begin: i32,
        finish: i32,
        dir: i32
    ) -> ExactMove {
        return ExactMove {
            start_x,
            start_z,
            end_x,
            end_z,
            begin,
            finish,
            dir,
        }
    }
}

impl Chat {
    #[inline]
    pub const fn new(
        bytes: Vec<u8>,
        color: u8,
        effect: u8,
        ignored: u8
    ) -> Chat {
        return Chat {
            bytes,
            color,
            effect,
            ignored,
        }
    }
}