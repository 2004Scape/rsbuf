use std::collections::HashMap;
use crate::message::{InfoMessage, PlayerInfoAnim, PlayerInfoAppearance, PlayerInfoFaceCoord, PlayerInfoFaceEntity};
use crate::packet::Packet;
use crate::player::Player;
use crate::prot::PlayerInfoProt;

pub struct PlayerRenderer {
    pub caches: HashMap<PlayerInfoProt, HashMap<i32, Vec<u8>>>,
    pub highs: [usize; 2048],
    pub lows: [usize; 2048],
}

impl PlayerRenderer {
    pub fn new() -> PlayerRenderer {
        let mut caches: HashMap<PlayerInfoProt, HashMap<i32, Vec<u8>>> = HashMap::with_capacity(10);
        caches.insert(PlayerInfoProt::Appearance, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::Anim, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::FaceEntity, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::Say, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::Damage, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::FaceCoord, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::Chat, HashMap::with_capacity(2048));
        caches.insert(PlayerInfoProt::SpotAnim, HashMap::with_capacity(2048));
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
        if masks & PlayerInfoProt::FaceCoord as u32 != 0 {
            let len: usize = self.cache(
                pid,
                &PlayerInfoFaceCoord::new(player.face_x, player.face_z),
                PlayerInfoProt::FaceCoord,
            );
            highs += len;
            lows += len;
        }

        if highs > 0 {
            unsafe { *self.highs.as_mut_ptr().add(pid as usize) = highs + PlayerRenderer::header(masks); }
        }

        if lows > 0 {
            let header: usize = PlayerRenderer::header(PlayerInfoProt::Appearance as u32 + PlayerInfoProt::FaceEntity as u32 + PlayerInfoProt::FaceCoord as u32);
            let appearance = self.caches
                .get(&PlayerInfoProt::Appearance)
                .and_then(|x| x.get(&pid))
                .map_or(0, |y| y.len());
            unsafe { *self.lows.as_mut_ptr().add(pid as usize) = header + appearance + 2 + 4; } // TODO? hardcoded lengths
        }
    }

    #[inline(always)]
    pub fn cache(&mut self, id: i32, message: &dyn InfoMessage, prot: PlayerInfoProt) -> usize {
        let cache: &mut HashMap<i32, Vec<u8>> = self.caches.get_mut(&prot).expect("Tried to cache to empty!");
        if cache.contains_key(&id) && !message.persists() {
            return 0;
        }
        return PlayerRenderer::encode_info(cache, id, message);
    }

    #[inline(always)]
    pub fn write(&self, buf: &mut Packet, id: i32, prot: PlayerInfoProt) {
        let cache: &HashMap<i32, Vec<u8>> = self.caches.get(&prot).expect("Tried to write a buf not cached!");
        self.write_block(buf, cache, id);
    }

    #[inline(always)]
    pub fn has(&self, id: i32, prot: PlayerInfoProt) -> bool {
        return if let Some(cache) = self.caches.get(&prot) {
            cache.contains_key(&id)
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn highdefinitions(&self, id: i32) -> usize {
        return unsafe { *self.highs.as_ptr().add(id as usize) };
    }

    #[inline(always)]
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
                cache.clear();
            }
        }
    }

    pub fn removePermanent(&mut self, id: i32) {
        unsafe {
            *self.highs.as_mut_ptr().add(id as usize) = 0;
            *self.lows.as_mut_ptr().add(id as usize) = 0;
        }
        if let Some(cache) = self.caches.get_mut(&PlayerInfoProt::Appearance) {
            cache.remove(&id);
        }
    }

    #[inline(always)]
    fn write_block(&self, buf: &mut Packet, messages: &HashMap<i32, Vec<u8>>, id: i32) {
        let mut bytes: &Vec<u8> = messages.get(&id).expect("Tried to write empty block!");
        buf.pdata(&bytes, 0, bytes.len());
    }

    // ----

    #[inline(always)]
    pub fn encode_info(messages: &mut HashMap<i32, Vec<u8>>, id: i32, message: &dyn InfoMessage) -> usize {
        let mut buf: Packet = Packet::new(message.test());
        message.encode(&mut buf);
        let len: usize = buf.len();
        messages.insert(id, buf.data);
        return len;
    }

    #[inline(always)]
    fn header(masks: u32) -> usize {
        let mut len: usize = 1;
        if masks > 0xff {
            len += 1;
        }
        return len;
    }
}