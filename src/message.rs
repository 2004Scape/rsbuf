use crate::packet::Packet;

pub trait InfoMessage {
    fn encode(&self, buf: &mut Packet);
    fn test(&self) -> usize;
    fn persists(&self) -> bool;
}

// ---- players

pub struct PlayerInfoAppearance {
    pub bytes: Vec<u8>,
}

impl PlayerInfoAppearance {
    #[inline]
    pub fn new(bytes: Vec<u8>) -> PlayerInfoAppearance {
        return PlayerInfoAppearance {
            bytes,
        };
    }
}

impl InfoMessage for PlayerInfoAppearance {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.bytes.len() as i32);
        buf.pdata(&self.bytes, 0, self.bytes.len());
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + self.bytes.len();
    }

    #[inline]
    fn persists(&self) -> bool {
        return true;
    }
}

// ----

pub struct PlayerInfoFaceEntity {
    pub entity: i32,
}

impl PlayerInfoFaceEntity {
    #[inline]
    pub fn new(entity: i32) -> PlayerInfoFaceEntity {
        return PlayerInfoFaceEntity {
            entity,
        }
    }
}

impl InfoMessage for PlayerInfoFaceEntity {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.entity);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoFaceCoord {
    pub x: i32,
    pub z: i32,
}

impl PlayerInfoFaceCoord {
    #[inline]
    pub fn new(
        x: i32,
        z: i32
    ) -> PlayerInfoFaceCoord {
        return PlayerInfoFaceCoord {
            x,
            z,
        }
    }
}

impl InfoMessage for PlayerInfoFaceCoord {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.x);
        buf.p2(self.z);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoAnim {
    pub anim: i32,
    pub delay: i32,
}

impl PlayerInfoAnim {
    #[inline]
    pub fn new(
        anim: i32,
        delay: i32
    ) -> PlayerInfoAnim {
        return PlayerInfoAnim {
            anim,
            delay,
        }
    }
}

impl InfoMessage for PlayerInfoAnim {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.anim);
        buf.p1(self.delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoSay {
    pub say: String
}

impl PlayerInfoSay {
    pub fn new(say: String) -> PlayerInfoSay {
        return PlayerInfoSay {
            say,
        }
    }
}

impl InfoMessage for PlayerInfoSay {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.say, 10);
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + self.say.len();
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoDamage {
    pub damage: i32,
    pub damage_type: i32,
    pub current_hitpoints: i32,
    pub base_hitpoints: i32,
}

impl PlayerInfoDamage {
    pub fn new(
        damage: i32,
        damage_type: i32,
        current_hitpoints: i32,
        base_hitpoints: i32,
    ) -> PlayerInfoDamage {
        return PlayerInfoDamage {
            damage,
            damage_type,
            current_hitpoints,
            base_hitpoints,
        }
    }
}

impl InfoMessage for PlayerInfoDamage {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.damage);
        buf.p1(self.damage_type);
        buf.p1(self.current_hitpoints);
        buf.p1(self.base_hitpoints);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoChat {
    bytes: Vec<u8>,
    color: i32,
    effect: i32,
    ignored: i32,
}

impl PlayerInfoChat {
    pub fn new(
        bytes: Vec<u8>,
        color: i32,
        effect: i32,
        ignored: i32
    ) -> PlayerInfoChat {
        return PlayerInfoChat {
            bytes,
            color,
            effect,
            ignored,
        }
    }
}

impl InfoMessage for PlayerInfoChat {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.color);
        buf.p1(self.effect);
        buf.p1(self.ignored);
        buf.p1(self.bytes.len() as i32);
        buf.pdata(&self.bytes, 0, self.bytes.len());
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + 1 + 1 + 1 + self.bytes.len();
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoSpotanim {
    pub graphic_id: i32,
    pub graphic_height: i32,
    pub graphic_delay: i32,
}

impl PlayerInfoSpotanim {
    pub fn new(
        graphic_id: i32,
        graphic_height: i32,
        graphic_delay: i32
    ) -> PlayerInfoSpotanim {
        return PlayerInfoSpotanim {
            graphic_id,
            graphic_height,
            graphic_delay,
        }
    }
}

impl InfoMessage for PlayerInfoSpotanim {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.graphic_id);
        buf.p4((self.graphic_height << 16) | self.graphic_delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct PlayerInfoExactMove {
    pub start_x: i32,
    pub start_z: i32,
    pub end_x: i32,
    pub end_z: i32,
    pub begin: i32,
    pub finish: i32,
    pub dir: i32
}

impl PlayerInfoExactMove {
    pub fn new(
        start_x: i32,
        start_z: i32,
        end_x: i32,
        end_z: i32,
        begin: i32,
        finish: i32,
        dir: i32,
    ) -> PlayerInfoExactMove {
        return PlayerInfoExactMove {
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

impl InfoMessage for PlayerInfoExactMove {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.start_x);
        buf.p1(self.start_z);
        buf.p1(self.end_x);
        buf.p1(self.end_z);
        buf.p2(self.begin);
        buf.p2(self.finish);
        buf.p1(self.dir);
    }

    #[inline]
    fn test(&self) -> usize {
        return 9;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ---- npcs

pub struct NpcInfoFaceEntity {
    pub entity: i32,
}

impl NpcInfoFaceEntity {
    #[inline]
    pub fn new(entity: i32) -> NpcInfoFaceEntity {
        return NpcInfoFaceEntity {
            entity,
        }
    }
}

impl InfoMessage for NpcInfoFaceEntity {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.entity);
    }

    #[inline]
    fn test(&self) -> usize {
        return 2;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoFaceCoord {
    pub x: i32,
    pub z: i32,
}

impl NpcInfoFaceCoord {
    #[inline]
    pub fn new(
        x: i32,
        z: i32
    ) -> NpcInfoFaceCoord {
        return NpcInfoFaceCoord {
            x,
            z,
        }
    }
}

impl InfoMessage for NpcInfoFaceCoord {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.x);
        buf.p2(self.z);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoAnim {
    pub anim: i32,
    pub delay: i32,
}

impl NpcInfoAnim {
    #[inline]
    pub fn new(
        anim: i32,
        delay: i32
    ) -> NpcInfoAnim {
        return NpcInfoAnim {
            anim,
            delay,
        }
    }
}

impl InfoMessage for NpcInfoAnim {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.anim);
        buf.p1(self.delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 3;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoSay {
    pub say: String
}

impl NpcInfoSay {
    pub fn new(say: String) -> NpcInfoSay {
        return NpcInfoSay {
            say,
        }
    }
}

impl InfoMessage for NpcInfoSay {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.say, 10);
    }

    #[inline]
    fn test(&self) -> usize {
        return 1 + self.say.len();
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoDamage {
    pub damage: i32,
    pub damage_type: i32,
    pub current_hitpoints: i32,
    pub base_hitpoints: i32,
}

impl NpcInfoDamage {
    pub fn new(
        damage: i32,
        damage_type: i32,
        current_hitpoints: i32,
        base_hitpoints: i32,
    ) -> NpcInfoDamage {
        return NpcInfoDamage {
            damage,
            damage_type,
            current_hitpoints,
            base_hitpoints,
        }
    }
}

impl InfoMessage for NpcInfoDamage {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.damage);
        buf.p1(self.damage_type);
        buf.p1(self.current_hitpoints);
        buf.p1(self.base_hitpoints);
    }

    #[inline]
    fn test(&self) -> usize {
        return 4;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoChangeType {
    pub change_type: i32,
}

impl NpcInfoChangeType {
    pub fn new(change_type: i32) -> NpcInfoChangeType {
        return NpcInfoChangeType {
            change_type,
        }
    }
}

impl InfoMessage for NpcInfoChangeType {
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.change_type);
    }

    fn test(&self) -> usize {
        return 2;
    }

    fn persists(&self) -> bool {
        return false;
    }
}

// ----

pub struct NpcInfoSpotanim {
    pub graphic_id: i32,
    pub graphic_height: i32,
    pub graphic_delay: i32,
}

impl NpcInfoSpotanim {
    pub fn new(
        graphic_id: i32,
        graphic_height: i32,
        graphic_delay: i32
    ) -> NpcInfoSpotanim {
        return NpcInfoSpotanim {
            graphic_id,
            graphic_height,
            graphic_delay,
        }
    }
}

impl InfoMessage for NpcInfoSpotanim {
    #[inline]
    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.graphic_id);
        buf.p4((self.graphic_height << 16) | self.graphic_delay);
    }

    #[inline]
    fn test(&self) -> usize {
        return 6;
    }

    #[inline]
    fn persists(&self) -> bool {
        return false;
    }
}

// ----