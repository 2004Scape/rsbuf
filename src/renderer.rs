use crate::message::{InfoMessage, NpcInfoAnim, NpcInfoChangeType, NpcInfoDamage, NpcInfoDamage2, NpcInfoFaceCoord, NpcInfoFaceEntity, NpcInfoSay, NpcInfoSpotanim, PlayerInfoAnim, PlayerInfoAppearance, PlayerInfoChat, PlayerInfoDamage, PlayerInfoDamage2, PlayerInfoExactMove, PlayerInfoFaceCoord, PlayerInfoFaceEntity, PlayerInfoSay, PlayerInfoSpotanim};
use crate::npc::Npc;
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::{NpcInfoProt, PlayerInfoProt};

pub struct PlayerRenderer {
    caches: Vec<Vec<Option<Vec<u8>>>>,
    highs: [usize; 2048],
    lows: [usize; 2048],
}

impl PlayerRenderer {
    #[inline]
    pub fn new() -> PlayerRenderer {
        // exact move does not get cached, that is built on demand.
        return PlayerRenderer {
            caches: vec![vec![None; 2048]; 9],
            highs: [0; 2048],
            lows: [0; 2048],
        }
    }

    #[inline]
    pub fn compute_info(&mut self, player: &Player) {
        let masks: u32 = player.masks;
        let pid: i32 = player.pid;

        if pid == -1 || masks == 0 {
            return;
        }

        let mut highs: usize = 0;
        let mut lows: usize = 0;

        // the ordering here does not matter.

        if masks & PlayerInfoProt::APPEARANCE as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoAppearance::new(player.appearance.clone()),
                PlayerInfoProt::APPEARANCE,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::ANIM as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoAnim::new(player.anim_id, player.anim_delay),
                PlayerInfoProt::ANIM,
            );
        }
        if masks & PlayerInfoProt::FACE_ENTITY as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoFaceEntity::new(player.face_entity),
                PlayerInfoProt::FACE_ENTITY,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::SAY as u32 != 0 {
            if let Some(say) = &player.say {
                highs += self.cache(
                    pid,
                    &PlayerInfoSay::new(say.clone()),
                    PlayerInfoProt::SAY,
                );
            }
        }
        if masks & PlayerInfoProt::DAMAGE as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoDamage::new(
                    player.damage_taken,
                    player.damage_type,
                    player.current_hitpoints,
                    player.base_hitpoints,
                ),
                PlayerInfoProt::DAMAGE,
            );
        }
        if masks & PlayerInfoProt::DAMAGE2 as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoDamage2::new(
                    player.damage_taken2,
                    player.damage_type2,
                    player.current_hitpoints,
                    player.base_hitpoints,
                ),
                PlayerInfoProt::DAMAGE2,
            );
        }
        if masks & PlayerInfoProt::FACE_COORD as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoFaceCoord::new(player.face_x, player.face_z),
                PlayerInfoProt::FACE_COORD,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::CHAT as u32 != 0 {
            if let Some(chat) = &player.chat {
                highs += self.cache(
                    pid,
                    &PlayerInfoChat::new(
                        chat.bytes.clone(),
                        chat.color as i32,
                        chat.effect as i32,
                        chat.ignored as i32,
                    ),
                    PlayerInfoProt::CHAT,
                );
            }
        }
        if masks & PlayerInfoProt::SPOT_ANIM as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoSpotanim::new(
                    player.graphic_id,
                    player.graphic_height,
                    player.graphic_delay,
                ),
                PlayerInfoProt::SPOT_ANIM,
            );
        }
        if masks & PlayerInfoProt::EXACT_MOVE as u32 != 0 {
            highs += 9;
        }

        if highs > 0 {
            unsafe { *self.highs.as_mut_ptr().add(pid as usize) = highs + PlayerRenderer::header(masks); }
        }

        if lows > 0 {
            let header: usize = PlayerRenderer::header(PlayerInfoProt::APPEARANCE as u32 + PlayerInfoProt::FACE_ENTITY as u32 + PlayerInfoProt::FACE_COORD as u32);
            unsafe {
                let appearance = (&(*self.caches.as_ptr().add(PlayerInfoProt::APPEARANCE.to_index())))
                    .get(pid as usize)
                    .and_then(|x| x.as_ref())
                    .map_or(0, |y| y.len());
                *self.lows.as_mut_ptr().add(pid as usize) = header + appearance + 2 + 4; // TODO? hardcoded lengths
            }
        }
    }

    #[inline]
    pub fn writeExactmove(
        &self,
        buf: &mut Packet,
        start_x: i32,
        start_z: i32,
        end_x: i32,
        end_z: i32,
        begin: i32,
        finish: i32,
        dir: i32,
    ) {
        PlayerInfoExactMove::new(start_x, start_z, end_x, end_z, begin, finish, dir).encode(buf);
    }

    #[inline]
    pub fn cache(&mut self, id: i32, message: &dyn InfoMessage, prot: PlayerInfoProt) -> usize {
        unsafe {
            let cache: &mut Vec<Option<Vec<u8>>> = &mut *self.caches.as_mut_ptr().add(prot.to_index());
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
            return PlayerRenderer::encode_info(cache, id, message);
        }
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: PlayerInfoProt) {
        unsafe {
            match &*(&*self.caches.as_ptr().add(prot.to_index())).as_ptr().add(id as usize) {
                Some(bytes) => buf.pdata(bytes, 0, bytes.len()),
                _ => panic!("[PlayerRenderer] Tried to write a buf not cached!"),
            }
        }
    }


    #[inline]
    pub fn has(&self, id: i32, prot: PlayerInfoProt) -> bool {
        return unsafe { (*(&*self.caches.as_ptr().add(prot.to_index())).as_ptr().add(id as usize)).is_some() };
    }


    #[inline]
    pub const fn highdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.highs.as_ptr().add(id as usize) };
    }

    #[inline]
    pub const fn lowdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.lows.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            PlayerInfoProt::ANIM.to_index(),
            PlayerInfoProt::FACE_ENTITY.to_index(),
            PlayerInfoProt::SAY.to_index(),
            PlayerInfoProt::DAMAGE.to_index(),
            PlayerInfoProt::DAMAGE2.to_index(),
            PlayerInfoProt::FACE_COORD.to_index(),
            PlayerInfoProt::CHAT.to_index(),
            PlayerInfoProt::SPOT_ANIM.to_index(),
        ] {
            unsafe { (*self.caches.as_mut_ptr().add(prot)).fill(None); }
        }
    }

    #[inline]
    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
            *(*self.caches.as_mut_ptr().add(PlayerInfoProt::APPEARANCE.to_index())).as_mut_ptr().add(id as usize) = None
        }
    }

    // ----

    #[inline]
    pub fn encode_info(messages: &mut Vec<Option<Vec<u8>>>, id: i32, message: &dyn InfoMessage) -> usize {
        let mut buf: Packet = Packet::new(message.test());
        message.encode(&mut buf);
        let len: usize = buf.len();
        unsafe { *messages.as_mut_ptr().add(id as usize) = Some(buf.data) };
        return len;
    }

    #[inline]
    const fn header(masks: u32) -> usize {
        let mut len: usize = 1;
        if masks > 0xff {
            len += 1;
        }
        return len;
    }
}

pub struct NpcRenderer {
    caches: Vec<Vec<Option<Vec<u8>>>>,
    highs: [usize; 8192],
    lows: [usize; 8192],
}

impl NpcRenderer {
    #[inline]
    pub fn new() -> NpcRenderer {
        return NpcRenderer {
            caches: vec![vec![None; 8192]; 8],
            highs: [0; 8192],
            lows: [0; 8192],
        }
    }

    #[inline]
    pub fn compute_info(&mut self, npc: &Npc) {
        let masks: u32 = npc.masks;
        let nid: i32 = npc.nid;

        if nid == -1 || masks == 0 {
            return;
        }

        let mut highs: usize = 0;
        let mut lows: usize = 0;

        // the ordering here does not matter.

        if masks & NpcInfoProt::ANIM as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoAnim::new(npc.anim_id, npc.anim_delay),
                NpcInfoProt::ANIM,
            );
        }
        if masks & NpcInfoProt::FACE_ENTITY as u32 != 0 {
            let len: usize = self.cache(
                nid,
                &NpcInfoFaceEntity::new(npc.face_entity),
                NpcInfoProt::FACE_ENTITY,
            );
            highs += len;
            lows += len;
        }
        if masks & NpcInfoProt::SAY as u32 != 0 {
            if let Some(say) = &npc.say {
                highs += self.cache(
                    nid,
                    &NpcInfoSay::new(say.clone()),
                    NpcInfoProt::SAY,
                );
            }
        }
        if masks & NpcInfoProt::DAMAGE as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoDamage::new(
                    npc.damage_taken,
                    npc.damage_type,
                    npc.current_hitpoints,
                    npc.base_hitpoints,
                ),
                NpcInfoProt::DAMAGE,
            );
        }
        if masks & NpcInfoProt::DAMAGE2 as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoDamage2::new(
                    npc.damage_taken2,
                    npc.damage_type2,
                    npc.current_hitpoints,
                    npc.base_hitpoints,
                ),
                NpcInfoProt::DAMAGE2,
            );
        }
        if masks & NpcInfoProt::CHANGE_TYPE as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoChangeType::new(npc.ntype),
                NpcInfoProt::CHANGE_TYPE,
            );
        }
        if masks & NpcInfoProt::SPOT_ANIM as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoSpotanim::new(
                    npc.graphic_id,
                    npc.graphic_height,
                    npc.graphic_delay,
                ),
                NpcInfoProt::SPOT_ANIM,
            );
        }
        if masks & NpcInfoProt::FACE_COORD as u32 != 0 {
            let len: usize = self.cache(
                nid,
                &NpcInfoFaceCoord::new(npc.face_x, npc.face_z),
                NpcInfoProt::FACE_COORD,
            );
            highs += len;
            lows += len;
        }

        if highs > 0 {
            unsafe { *self.highs.as_mut_ptr().add(nid as usize) = highs + NpcRenderer::header(masks); }
        }

        if lows > 0 {
            let header: usize = NpcRenderer::header(NpcInfoProt::FACE_ENTITY as u32 + NpcInfoProt::FACE_COORD as u32);
            unsafe { *self.lows.as_mut_ptr().add(nid as usize) = header + 2 + 4; } // TODO? hardcoded lengths
        }
    }

    #[inline]
    pub fn cache(&mut self, id: i32, message: &dyn InfoMessage, prot: NpcInfoProt) -> usize {
        unsafe {
            let cache: &mut Vec<Option<Vec<u8>>> = &mut *self.caches.as_mut_ptr().add(prot.to_index());
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
            return NpcRenderer::encode_info(cache, id, message);
        }
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: NpcInfoProt) {
        unsafe {
            match &*(&*self.caches.as_ptr().add(prot.to_index())).as_ptr().add(id as usize) {
                Some(bytes) => buf.pdata(bytes, 0, bytes.len()),
                _ => panic!("[NpcRenderer] Tried to write a buf not cached!"),
            }
        }
    }


    #[inline]
    pub fn has(&self, id: i32, prot: NpcInfoProt) -> bool {
        return unsafe { (*(&*self.caches.as_ptr().add(prot.to_index())).as_ptr().add(id as usize)).is_some() };
    }


    #[inline]
    pub const fn highdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.highs.as_ptr().add(id as usize) };
    }

    #[inline]
    pub const fn lowdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.lows.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            NpcInfoProt::ANIM.to_index(),
            NpcInfoProt::FACE_ENTITY.to_index(),
            NpcInfoProt::SAY.to_index(),
            NpcInfoProt::DAMAGE.to_index(),
            NpcInfoProt::DAMAGE2.to_index(),
            NpcInfoProt::CHANGE_TYPE.to_index(),
            NpcInfoProt::SPOT_ANIM.to_index(),
            NpcInfoProt::FACE_COORD.to_index(),
        ] {
            unsafe { (*self.caches.as_mut_ptr().add(prot)).fill(None); }
        }
    }

    #[inline]
    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
        }
    }

    // ----

    #[inline]
    pub fn encode_info(messages: &mut Vec<Option<Vec<u8>>>, id: i32, message: &dyn InfoMessage) -> usize {
        let mut buf: Packet = Packet::new(message.test());
        message.encode(&mut buf);
        let len: usize = buf.len();
        unsafe { *messages.as_mut_ptr().add(id as usize) = Some(buf.data) };
        return len;
    }

    #[inline]
    const fn header(masks: u32) -> usize {
        let mut len: usize = 1;
        if masks > 0xff {
            len += 1;
        }
        return len;
    }
}