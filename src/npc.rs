use crate::coord::CoordGrid;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Npc {
    pub coord: CoordGrid,
    pub nid: i32,
    pub ntype: i32,
    pub tele: bool,
    pub run_dir: i8,
    pub walk_dir: i8,
    pub lifecycle: u8,
    pub lifecycle_tick: i32,
    pub masks: u32,
    pub face_entity: i32,
    pub face_x: i32,
    pub face_z: i32,
    pub orientation_x: i32,
    pub orientation_z: i32,
    pub damage_taken: i32,
    pub damage_type: i32,
    pub current_hitpoints: i32,
    pub base_hitpoints: i32,
    pub anim_id: i32,
    pub anim_delay: i32,
    pub say: Option<String>,
    pub graphic_id: i32,
    pub graphic_height: i32,
    pub graphic_delay: i32,
}

impl Npc {
    pub fn new(nid: i32, ntype: i32) -> Self {
        return Npc {
            coord: CoordGrid::from(0, 0, 0),
            nid,
            ntype,
            tele: false,
            run_dir: -1,
            walk_dir: -1,
            lifecycle: 0,
            lifecycle_tick: 0,
            masks: 0,
            face_entity: -1,
            face_x: -1,
            face_z: -1,
            orientation_x: -1,
            orientation_z: -1,
            damage_taken: -1,
            damage_type: -1,
            current_hitpoints: -1,
            base_hitpoints: -1,
            anim_id: -1,
            anim_delay: -1,
            say: None,
            graphic_id: -1,
            graphic_height: -1,
            graphic_delay: -1,
        }
    }

    #[inline]
    pub fn check_life_cycle(&self, tick: u32) -> bool {
        return match self.lifecycle {
            0 => true,
            1 => self.lifecycle_tick < tick as i32,
            _ => self.lifecycle_tick > tick as i32,
        };
    }

    #[inline]
    pub fn cleanup(&mut self) {
        self.walk_dir = -1;
        self.run_dir = -1;
        self.tele = false;
        self.masks = 0;
        // self.face_entity = -1;
        self.face_x = -1;
        self.face_z = -1;
        // self.orientation_x = -1;
        // self.orientation_z = -1;
        self.damage_taken = -1;
        self.damage_type = -1;
        self.current_hitpoints = -1;
        self.base_hitpoints = -1;
        self.anim_id = -1;
        self.anim_delay = -1;
        self.say = None;
        self.graphic_id = -1;
        self.graphic_height = -1;
        self.graphic_delay = -1;
    }
}