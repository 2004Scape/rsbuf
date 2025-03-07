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
    pub fn new() -> PlayerRenderer {
        fn empty_array() -> [Option<Vec<u8>>; 2048] {
            return std::array::from_fn(|_| None)
        }

        let mut caches: HashMap<PlayerInfoProt, [Option<Vec<u8>>; 2048]> = HashMap::with_capacity(10);
        caches.insert(PlayerInfoProt::Appearance, empty_array());
        caches.insert(PlayerInfoProt::Anim, empty_array());
        caches.insert(PlayerInfoProt::FaceEntity, empty_array());
        caches.insert(PlayerInfoProt::Say, empty_array());
        caches.insert(PlayerInfoProt::Damage, empty_array());
        caches.insert(PlayerInfoProt::FaceCoord, empty_array());
        caches.insert(PlayerInfoProt::Chat, empty_array());
        caches.insert(PlayerInfoProt::SpotAnim, empty_array());
        // exact move does not get cached, that is built on demand.
        return PlayerRenderer {
            caches,
            highs: [0; 2048],
            lows: [0; 2048],
        }
    }

    pub fn compute_info(&mut self, player: &Player) {
        let masks: u32 = player.masks;
        let pid: i32 = player.pid;

        if pid == -1 || masks == 0 {
            return;
        }

        let mut highs: usize = 0;
        let mut lows: usize = 0;

        if masks & PlayerInfoProt::Appearance as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoAppearance::new(player.appearance.clone()),
                PlayerInfoProt::Appearance,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::Anim as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoAnim::new(player.anim_id, player.anim_delay),
                PlayerInfoProt::Anim,
            );
        }
        if masks & PlayerInfoProt::FaceEntity as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoFaceEntity::new(player.face_entity),
                PlayerInfoProt::FaceEntity,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::Say as u32 != 0 {
            if let Some(say) = &player.say {
                highs += self.cache(
                    pid,
                    &PlayerInfoSay::new(say.clone()),
                    PlayerInfoProt::Say,
                );
            }
        }
        if masks & PlayerInfoProt::Damage as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoDamage::new(
                    player.damage_taken,
                    player.damage_type,
                    player.current_hitpoints,
                    player.base_hitpoints,
                ),
                PlayerInfoProt::Damage,
            );
        }
        if masks & PlayerInfoProt::FaceCoord as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoFaceCoord::new(player.face_x, player.face_z),
                PlayerInfoProt::FaceCoord,
            );
            highs += len;
            lows += len;
        }
        if masks & PlayerInfoProt::Chat as u32 != 0 {
            if let Some(chat) = &player.chat {
                highs += self.cache(
                    pid,
                    &PlayerInfoChat::new(
                        chat.bytes.clone(),
                        chat.color as i32,
                        chat.effect as i32,
                        chat.ignored as i32,
                    ),
                    PlayerInfoProt::Chat,
                );
            }
        }
        if masks & PlayerInfoProt::SpotAnim as u32 != 0 {
            highs += self.cache(
                pid,
                &PlayerInfoSpotanim::new(
                    player.graphic_id,
                    player.graphic_height,
                    player.graphic_delay,
                ),
                PlayerInfoProt::SpotAnim,
            );
        }
        if masks & PlayerInfoProt::ExactMove as u32 != 0 {
            highs += 9;
        }

        if highs > 0 {
            unsafe { *self.highs.as_mut_ptr().add(pid as usize) = highs + PlayerRenderer::header(masks); }
        }

        if lows > 0 {
            let header: usize = PlayerRenderer::header(PlayerInfoProt::Appearance as u32 + PlayerInfoProt::FaceEntity as u32 + PlayerInfoProt::FaceCoord as u32);
            let appearance = self.caches[&PlayerInfoProt::Appearance]
                .get(pid as usize)
                .and_then(|x| x.as_ref())
                .map_or(0, |y| y.len());
            unsafe { *self.lows.as_mut_ptr().add(pid as usize) = header + appearance + 2 + 4; } // TODO? hardcoded lengths
        }
    }

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
        let cache: &mut [Option<Vec<u8>>; 2048] = self.caches.get_mut(&prot).expect("");
        unsafe {
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
        }
        return PlayerRenderer::encode_info(cache, id, message);
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: PlayerInfoProt) {
        let cache: &[Option<Vec<u8>>; 2048] = self.caches.get(&prot).expect("");
        unsafe {
            if let Some(bytes) = &*cache.as_ptr().add(id as usize) {
                self.write_block(buf, bytes, id);
            } else {
                panic!("Tried to write a buf not cached!");
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

    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            PlayerInfoProt::Anim,
            PlayerInfoProt::FaceEntity,
            PlayerInfoProt::Say,
            PlayerInfoProt::Damage,
            PlayerInfoProt::FaceCoord,
            PlayerInfoProt::Chat,
            PlayerInfoProt::SpotAnim,
        ] {
            if let Some(cache) = self.caches.get_mut(&prot) {
                cache.fill(None);
            }
        }
    }

    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
        }
        if let Some(cache) = self.caches.get_mut(&PlayerInfoProt::Appearance) {
            unsafe { *cache.as_mut_ptr().add(id as usize) = None }
        }
    }

    #[inline]
    fn write_block(&self, buf: &mut Packet, messages: &Vec<u8>, id: i32) {
        // let bytes = messages.get(id as usize).and_then(|opt| opt.as_ref()).expect("Tried to write empty block!");
        buf.pdata(messages, 0, messages.len());
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
    pub fn new() -> NpcRenderer {
        fn empty_array() -> [Option<Vec<u8>>; 8192] {
            return std::array::from_fn(|_| None)
        }

        let mut caches: HashMap<NpcInfoProt, [Option<Vec<u8>>; 8192]> = HashMap::with_capacity(10);
        caches.insert(NpcInfoProt::Anim, empty_array());
        caches.insert(NpcInfoProt::FaceEntity, empty_array());
        caches.insert(NpcInfoProt::Say, empty_array());
        caches.insert(NpcInfoProt::Damage, empty_array());
        caches.insert(NpcInfoProt::ChangeType, empty_array());
        caches.insert(NpcInfoProt::SpotAnim, empty_array());
        caches.insert(NpcInfoProt::FaceCoord, empty_array());
        return NpcRenderer {
            caches,
            highs: [0; 8192],
            lows: [0; 8192],
        }
    }

    pub fn compute_info(&mut self, npc: &Npc) {
        let masks: u32 = npc.masks;
        let nid: i32 = npc.nid;

        if nid == -1 || masks == 0 {
            return;
        }

        let mut highs: usize = 0;
        let mut lows: usize = 0;

        if masks & NpcInfoProt::Anim as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoAnim::new(npc.anim_id, npc.anim_delay),
                NpcInfoProt::Anim,
            );
        }
        if masks & NpcInfoProt::FaceEntity as u32 != 0 {
            let len: usize = self.cache(
                nid,
                &NpcInfoFaceEntity::new(npc.face_entity),
                NpcInfoProt::FaceEntity,
            );
            highs += len;
            lows += len;
        }
        if masks & NpcInfoProt::Say as u32 != 0 {
            if let Some(say) = &npc.say {
                highs += self.cache(
                    nid,
                    &NpcInfoSay::new(say.clone()),
                    NpcInfoProt::Say,
                );
            }
        }
        if masks & NpcInfoProt::Damage as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoDamage::new(
                    npc.damage_taken,
                    npc.damage_type,
                    npc.current_hitpoints,
                    npc.base_hitpoints,
                ),
                NpcInfoProt::Damage,
            );
        }
        if masks & NpcInfoProt::ChangeType as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoChangeType::new(npc.ntype),
                NpcInfoProt::ChangeType,
            );
        }
        if masks & NpcInfoProt::SpotAnim as u32 != 0 {
            highs += self.cache(
                nid,
                &NpcInfoSpotanim::new(
                    npc.graphic_id,
                    npc.graphic_height,
                    npc.graphic_delay,
                ),
                NpcInfoProt::SpotAnim,
            );
        }
        if masks & NpcInfoProt::FaceCoord as u32 != 0 {
            let len: usize = self.cache(
                nid,
                &NpcInfoFaceCoord::new(npc.face_x, npc.face_z),
                NpcInfoProt::FaceCoord,
            );
            highs += len;
            lows += len;
        }

        if highs > 0 {
            unsafe { *self.highs.as_mut_ptr().add(nid as usize) = highs + NpcRenderer::header(masks); }
        }

        if lows > 0 {
            let header: usize = NpcRenderer::header(NpcInfoProt::FaceEntity as u32 + NpcInfoProt::FaceCoord as u32);
            unsafe { *self.lows.as_mut_ptr().add(nid as usize) = header + 2 + 4; } // TODO? hardcoded lengths
        }
    }

    #[inline]
    pub fn cache(&mut self, id: i32, message: &dyn InfoMessage, prot: NpcInfoProt) -> usize {
        let cache: &mut [Option<Vec<u8>>; 8192] = self.caches.get_mut(&prot).expect("");
        unsafe {
            if (*cache.as_ptr().add(id as usize)).is_some() && !message.persists() {
                return 0;
            }
        }
        return NpcRenderer::encode_info(cache, id, message);
    }

    #[inline]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: NpcInfoProt) {
        let cache: &[Option<Vec<u8>>; 8192] = self.caches.get(&prot).expect("");
        unsafe {
            if let Some(bytes) = &*cache.as_ptr().add(id as usize) {
                self.write_block(buf, bytes, id);
            } else {
                panic!("Tried to write a buf not cached!");
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

    pub fn removeTemporary(&mut self) {
        self.highs.fill(0);
        for prot in [
            NpcInfoProt::Anim,
            NpcInfoProt::FaceEntity,
            NpcInfoProt::Say,
            NpcInfoProt::Damage,
            NpcInfoProt::ChangeType,
            NpcInfoProt::SpotAnim,
            NpcInfoProt::FaceCoord,
        ] {
            if let Some(cache) = self.caches.get_mut(&prot) {
                cache.fill(None);
            }
        }
    }

    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
        }
    }

    #[inline]
    fn write_block(&self, buf: &mut Packet, messages: &Vec<u8>, id: i32) {
        // let bytes = messages.get(id as usize).and_then(|opt| opt.as_ref()).expect("Tried to write empty block!");
        buf.pdata(messages, 0, messages.len());
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