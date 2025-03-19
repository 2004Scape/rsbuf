use std::array::from_fn;
use std::collections::HashMap;
use crate::message::{InfoMessage, NpcInfoAnim, NpcInfoChangeType, NpcInfoDamage, NpcInfoFaceCoord, NpcInfoFaceEntity, NpcInfoSay, NpcInfoSpotanim, PlayerInfoAnim, PlayerInfoAppearance, PlayerInfoChat, PlayerInfoDamage, PlayerInfoExactMove, PlayerInfoFaceCoord, PlayerInfoFaceEntity, PlayerInfoSay, PlayerInfoSpotanim};
use crate::npc::Npc;
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::{NpcInfoProt, PlayerInfoProt};

pub struct PlayerRenderer {
    pub caches: HashMap<PlayerInfoProt, [Option<Vec<u8>>; 2048]>,
    pub highs: [usize; 2048],
    pub lows: [usize; 2048],
}

impl PlayerRenderer {
    #[inline]
    pub fn new() -> PlayerRenderer {
        let mut caches: HashMap<PlayerInfoProt, [Option<Vec<u8>>; 2048]> = HashMap::with_capacity(8);
        caches.insert(PlayerInfoProt::APPEARANCE, from_fn(|_| None));
        caches.insert(PlayerInfoProt::ANIM, from_fn(|_| None));
        caches.insert(PlayerInfoProt::FACE_ENTITY, from_fn(|_| None));
        caches.insert(PlayerInfoProt::SAY, from_fn(|_| None));
        caches.insert(PlayerInfoProt::DAMAGE, from_fn(|_| None));
        caches.insert(PlayerInfoProt::FACE_COORD, from_fn(|_| None));
        caches.insert(PlayerInfoProt::CHAT, from_fn(|_| None));
        caches.insert(PlayerInfoProt::SPOT_ANIM, from_fn(|_| None));
        // exact move does not get cached, that is built on demand.
        return PlayerRenderer {
            caches,
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
            let appearance = self.caches[&PlayerInfoProt::APPEARANCE]
                .get(pid as usize)
                .and_then(|x| x.as_ref())
                .map_or(0, |y| y.len());
            unsafe { *self.lows.as_mut_ptr().add(pid as usize) = header + appearance + 2 + 4; } // TODO? hardcoded lengths
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
        let cache: &mut [Option<Vec<u8>>; 2048] = self.caches.get_mut(&prot).expect("[PlayerRenderer] Prot not found in cache!");
        unsafe {
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
        }
        return PlayerRenderer::encode_info(cache, id, message);
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: PlayerInfoProt) {
        let cache: &[Option<Vec<u8>>; 2048] = self.caches.get(&prot).expect("[PlayerRenderer] Prot not found in cache for write!");
        unsafe {
            if let Some(bytes) = &*cache.as_ptr().add(id as usize) {
                buf.pdata(bytes, 0, bytes.len());
            } else {
                panic!("[PlayerRenderer] Tried to write a buf not cached!");
            }
        }
    }


    #[inline]
    pub fn has(&self, id: i32, prot: PlayerInfoProt) -> bool {
        let cache: &[Option<Vec<u8>>; 2048] = &self.caches[&prot];
        unsafe { return (*cache.as_ptr().add(id as usize)).is_some(); }
    }


    #[inline]
    pub fn highdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.highs.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn lowdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.lows.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            PlayerInfoProt::ANIM,
            PlayerInfoProt::FACE_ENTITY,
            PlayerInfoProt::SAY,
            PlayerInfoProt::DAMAGE,
            PlayerInfoProt::FACE_COORD,
            PlayerInfoProt::CHAT,
            PlayerInfoProt::SPOT_ANIM,
        ] {
            if let Some(cache) = self.caches.get_mut(&prot) {
                cache.fill(None);
            }
        }
    }

    #[inline]
    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
        }
        if let Some(cache) = self.caches.get_mut(&PlayerInfoProt::APPEARANCE) {
            unsafe { *cache.as_mut_ptr().add(id as usize) = None }
        }
    }

    // ----

    #[inline]
    pub fn encode_info(messages: &mut [Option<Vec<u8>>; 2048], id: i32, message: &dyn InfoMessage) -> usize {
        let mut buf: Packet = Packet::new(message.test());
        message.encode(&mut buf);
        let len: usize = buf.len();
        messages[id as usize] = Some(buf.data);
        return len;
    }

    #[inline]
    fn header(masks: u32) -> usize {
        let mut len: usize = 1;
        if masks > 0xff {
            len += 1;
        }
        return len;
    }
}

pub struct NpcRenderer {
    pub caches: HashMap<NpcInfoProt, [Option<Vec<u8>>; 8192]>,
    pub highs: [usize; 8192],
    pub lows: [usize; 8192],
}

impl NpcRenderer {
    #[inline]
    pub fn new() -> NpcRenderer {
        let mut caches: HashMap<NpcInfoProt, [Option<Vec<u8>>; 8192]> = HashMap::with_capacity(7);
        caches.insert(NpcInfoProt::ANIM, from_fn(|_| None));
        caches.insert(NpcInfoProt::FACE_ENTITY, from_fn(|_| None));
        caches.insert(NpcInfoProt::SAY, from_fn(|_| None));
        caches.insert(NpcInfoProt::DAMAGE, from_fn(|_| None));
        caches.insert(NpcInfoProt::CHANGE_TYPE, from_fn(|_| None));
        caches.insert(NpcInfoProt::SPOT_ANIM, from_fn(|_| None));
        caches.insert(NpcInfoProt::FACE_COORD, from_fn(|_| None));
        return NpcRenderer {
            caches,
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
        let cache: &mut [Option<Vec<u8>>; 8192] = self.caches.get_mut(&prot).expect("[NpcRenderer] Prot not found in cache for write!");
        unsafe {
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
        }
        return NpcRenderer::encode_info(cache, id, message);
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: NpcInfoProt) {
        let cache: &[Option<Vec<u8>>; 8192] = self.caches.get(&prot).expect("[NpcRenderer] Prot not found in cache for write!");
        unsafe {
            if let Some(bytes) = &*cache.as_ptr().add(id as usize) {
                buf.pdata(bytes, 0, bytes.len());
            } else {
                panic!("[NpcRenderer] Tried to write a buf not cached!");
            }
        }
    }


    #[inline]
    pub fn has(&self, id: i32, prot: NpcInfoProt) -> bool {
        let cache: &[Option<Vec<u8>>; 8192] = &self.caches[&prot];
        unsafe { return (*cache.as_ptr().add(id as usize)).is_some(); }
    }


    #[inline]
    pub fn highdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.highs.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn lowdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.lows.as_ptr().add(id as usize) };
    }

    #[inline]
    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            NpcInfoProt::ANIM,
            NpcInfoProt::FACE_ENTITY,
            NpcInfoProt::SAY,
            NpcInfoProt::DAMAGE,
            NpcInfoProt::CHANGE_TYPE,
            NpcInfoProt::SPOT_ANIM,
            NpcInfoProt::FACE_COORD,
        ] {
            if let Some(cache) = self.caches.get_mut(&prot) {
                cache.fill(None);
            }
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
    pub fn encode_info(messages: &mut [Option<Vec<u8>>; 8192], id: i32, message: &dyn InfoMessage) -> usize {
        let mut buf: Packet = Packet::new(message.test());
        message.encode(&mut buf);
        let len: usize = buf.len();
        messages[id as usize] = Some(buf.data);
        return len;
    }

    #[inline]
    fn header(masks: u32) -> usize {
        let mut len: usize = 1;
        if masks > 0xff {
            len += 1;
        }
        return len;
    }
}