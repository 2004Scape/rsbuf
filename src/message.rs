use crate::packet::Packet;
use crate::priority::ServerProtPriority;
use crate::prot::{ClientProt, ServerInternalProt};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::coord::CoordGrid;

pub trait InfoMessage {
    fn encode(&self, buf: &mut Packet);
    fn test(&self) -> usize;
}

pub trait MessageEncoder {
    fn id(&self) -> i32;
    fn length(&self) -> i32;
    fn priority(&self) -> ServerProtPriority;
    fn encode(&self, buf: &mut Packet);
    fn test(&self) -> usize;
}

pub trait MessageDecoder<T> {
    fn length() -> i32;
    fn decode(prot: ClientProt, buf: &mut Packet) -> T;
}

#[wasm_bindgen]
#[derive(Clone)] // for VecDeque
pub struct OutgoingPacket {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub bytes: Option<Vec<u8>>,
    #[wasm_bindgen(readonly)]
    pub id: i32,
    #[wasm_bindgen(readonly)]
    pub length: i32,
}

impl OutgoingPacket {
    pub fn new(
        bytes: Option<Vec<u8>>,
        id: i32,
        length: i32
    ) -> OutgoingPacket {
        return OutgoingPacket {
            bytes,
            id,
            length,
        }
    }
}

#[wasm_bindgen]
pub struct IncomingPacket {
    #[wasm_bindgen(readonly)]
    pub id: i32,
    #[wasm_bindgen(readonly)]
    pub length: i32,
}

impl IncomingPacket {
    pub fn new(
        id: i32,
        length: i32
    ) -> IncomingPacket {
        return IncomingPacket {
            id,
            length,
        }
    }
}

// ---- players

pub struct PlayerInfoAppearance {
    bytes: Vec<u8>,
}

impl PlayerInfoAppearance {
    #[inline]
    pub const fn new(bytes: Vec<u8>) -> PlayerInfoAppearance {
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
}

// ----

pub struct PlayerInfoFaceEntity {
    entity: i32,
}

impl PlayerInfoFaceEntity {
    #[inline]
    pub const fn new(entity: i32) -> PlayerInfoFaceEntity {
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
}

// ----

pub struct PlayerInfoFaceCoord {
    x: i32,
    z: i32,
}

impl PlayerInfoFaceCoord {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct PlayerInfoAnim {
    anim: i32,
    delay: i32,
}

impl PlayerInfoAnim {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct PlayerInfoSay {
    say: String
}

impl PlayerInfoSay {
    #[inline]
    pub const fn new(say: String) -> PlayerInfoSay {
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
}

// ----

pub struct PlayerInfoDamage {
    damage: i32,
    damage_type: i32,
    current_hitpoints: i32,
    base_hitpoints: i32,
}

impl PlayerInfoDamage {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct PlayerInfoChat {
    bytes: Vec<u8>,
    color: i32,
    effect: i32,
    ignored: i32,
}

impl PlayerInfoChat {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct PlayerInfoSpotanim {
    graphic_id: i32,
    graphic_height: i32,
    graphic_delay: i32,
}

impl PlayerInfoSpotanim {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct PlayerInfoExactMove {
    start_x: i32,
    start_z: i32,
    end_x: i32,
    end_z: i32,
    begin: i32,
    finish: i32,
    dir: i32
}

impl PlayerInfoExactMove {
    #[inline]
    pub const fn new(
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
}

// ---- npcs

pub struct NpcInfoFaceEntity {
    entity: i32,
}

impl NpcInfoFaceEntity {
    #[inline]
    pub const fn new(entity: i32) -> NpcInfoFaceEntity {
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
}

// ----

pub struct NpcInfoFaceCoord {
    x: i32,
    z: i32,
}

impl NpcInfoFaceCoord {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct NpcInfoAnim {
    anim: i32,
    delay: i32,
}

impl NpcInfoAnim {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct NpcInfoSay {
    say: String
}

impl NpcInfoSay {
    #[inline]
    pub const fn new(say: String) -> NpcInfoSay {
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
}

// ----

pub struct NpcInfoDamage {
    damage: i32,
    damage_type: i32,
    current_hitpoints: i32,
    base_hitpoints: i32,
}

impl NpcInfoDamage {
    #[inline]
    pub const fn new(
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
}

// ----

pub struct NpcInfoChangeType {
    change_type: i32,
}

impl NpcInfoChangeType {
    #[inline]
    pub const fn new(change_type: i32) -> NpcInfoChangeType {
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
}

// ----

pub struct NpcInfoSpotanim {
    graphic_id: i32,
    graphic_height: i32,
    graphic_delay: i32,
}

impl NpcInfoSpotanim {
    #[inline]
    pub const fn new(
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
}

// ---- encoders

pub struct IfOpenMain {
    component: i32,
}

impl IfOpenMain {
    pub fn new(component:i32) -> IfOpenMain {
        return IfOpenMain {
            component,
        }
    }
}

impl MessageEncoder for IfOpenMain {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENMAIN as i32;
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    fn test(&self) -> usize {
        return 2;
    }
}

// ----

pub struct IfOpenSide {
    component: i32,
}

impl IfOpenSide {
    pub fn new(component:i32) -> IfOpenSide {
        return IfOpenSide {
            component,
        }
    }
}

impl MessageEncoder for IfOpenSide {
    fn id(&self) -> i32 {
        return ServerInternalProt::IF_OPENSIDE as i32
    }

    fn length(&self) -> i32 {
        return 2;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Buffered;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p2(self.component);
    }

    fn test(&self) -> usize {
        return 2;
    }
}

// ----

pub struct MessageGame {
    msg: String,
}

impl MessageGame {
    pub fn new(msg: String) -> MessageGame {
        return MessageGame {
            msg,
        }
    }
}

impl MessageEncoder for MessageGame {
    fn id(&self) -> i32 {
        return ServerInternalProt::MESSAGE_GAME as i32
    }

    fn length(&self) -> i32 {
        return -1;
    }

    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.pjstr(&self.msg, 10);
    }

    fn test(&self) -> usize {
        return 1 + self.msg.len();
    }
}

// ---- decoders

#[wasm_bindgen]
pub struct ClientCheat {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: String,
}

#[wasm_bindgen]
impl ClientCheat {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> ClientCheat {
        return ClientCheat {
            input,
        }
    }
}

impl MessageDecoder<ClientCheat> for ClientCheat {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ClientCheat {
        return ClientCheat::new(buf.gjstr(10));
    }
}

// ----

#[wasm_bindgen]
pub struct CloseModal {}

#[wasm_bindgen]
impl CloseModal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CloseModal {
        return CloseModal {}
    }
}

impl MessageDecoder<CloseModal> for CloseModal {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> CloseModal {
        return CloseModal::new();
    }
}

// ----

#[wasm_bindgen]
pub struct FriendListAdd {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl FriendListAdd {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> FriendListAdd {
        return FriendListAdd {
            username,
        }
    }
}

impl MessageDecoder<FriendListAdd> for FriendListAdd {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> FriendListAdd {
        return FriendListAdd::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct FriendListDel {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl FriendListDel {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> FriendListDel {
        return FriendListDel {
            username,
        }
    }
}

impl MessageDecoder<FriendListDel> for FriendListDel {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> FriendListDel {
        return FriendListDel::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct IdleTimer {}

#[wasm_bindgen]
impl IdleTimer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IdleTimer {
        return IdleTimer {}
    }
}

impl MessageDecoder<IdleTimer> for IdleTimer {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> IdleTimer {
        return IdleTimer::new();
    }
}

// ----

#[wasm_bindgen]
pub struct IfButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl IfButton {
    #[wasm_bindgen(constructor)]
    pub fn new(component: u16) -> IfButton {
        return IfButton {
            component,
        }
    }
}

impl MessageDecoder<IfButton> for IfButton {
    fn length() -> i32 {
        return 2;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> IfButton {
        return IfButton::new(buf.g2());
    }
}

// ----

#[wasm_bindgen]
pub struct IfPlayerDesign {
    #[wasm_bindgen(readonly)]
    pub gender: u8,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub idkit: Vec<i32>,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub color: Vec<i32>,
}

#[wasm_bindgen]
impl IfPlayerDesign {
    #[wasm_bindgen(constructor)]
    pub fn new(gender: u8, idkit: Vec<i32>, color: Vec<i32>) -> IfPlayerDesign {
        return IfPlayerDesign {
            gender,
            idkit,
            color,
        }
    }
}

impl MessageDecoder<IfPlayerDesign> for IfPlayerDesign {
    fn length() -> i32 {
        return 13;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> IfPlayerDesign {
        let gender: u8 = buf.g1();

        let mut idkit: [i32; 7] = [0; 7];
        for i in 0..7 {
            let mut v = buf.g1() as i32;
            if v == 0xff {
                v = -1;
            }
            unsafe { *idkit.as_mut_ptr().add(i as usize) = v };
        }

        let mut color: [i32; 5] = [0; 5];
        for i in 0..5 {
            unsafe { *color.as_mut_ptr().add(i as usize) = buf.g1() as i32 };
        }

        return IfPlayerDesign::new(gender, idkit.to_vec(), color.to_vec());
    }
}

// ----

#[wasm_bindgen]
pub struct IgnoreListAdd {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl IgnoreListAdd {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> IgnoreListAdd {
        return IgnoreListAdd {
            username,
        }
    }
}

impl MessageDecoder<IgnoreListAdd> for IgnoreListAdd {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> IgnoreListAdd {
        return IgnoreListAdd::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct IgnoreListDel {
    #[wasm_bindgen(readonly)]
    pub username: i64,
}

#[wasm_bindgen]
impl IgnoreListDel {
    #[wasm_bindgen(constructor)]
    pub fn new(username: i64) -> IgnoreListDel {
        return IgnoreListDel {
            username,
        }
    }
}

impl MessageDecoder<IgnoreListDel> for IgnoreListDel {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> IgnoreListDel {
        return IgnoreListDel::new(buf.g8s());
    }
}

// ----

#[wasm_bindgen]
pub struct InvButton {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl InvButton {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        obj: u16,
        slot: u16,
        component: u16,
    ) -> InvButton {
        return InvButton {
            op,
            obj,
            slot,
            component,
        }
    }
}

impl MessageDecoder<InvButton> for InvButton {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> InvButton {
        let op: u8 = match prot {
            ClientProt::INV_BUTTON1 => 1,
            ClientProt::INV_BUTTON2 => 2,
            ClientProt::INV_BUTTON3 => 3,
            ClientProt::INV_BUTTON4 => 4,
            ClientProt::INV_BUTTON5 => 5,
            _ => 0,
        };
        return InvButton::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct InvButtonD {
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub target: u16,
}

#[wasm_bindgen]
impl InvButtonD {
    #[wasm_bindgen(constructor)]
    pub fn new(
        component: u16,
        slot: u16,
        target: u16,
    ) -> InvButtonD {
        return InvButtonD {
            component,
            slot,
            target,
        }
    }
}

impl MessageDecoder<InvButtonD> for InvButtonD {
    fn length() -> i32 {
        return 6;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> InvButtonD {
        return InvButtonD::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct MessagePrivate {
    #[wasm_bindgen(readonly)]
    pub username: i64,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: Vec<u8>,
}

#[wasm_bindgen]
impl MessagePrivate {
    #[wasm_bindgen(constructor)]
    pub fn new(
        username: i64,
        input: Vec<u8>,
    ) -> MessagePrivate {
        return MessagePrivate {
            username,
            input,
        }
    }
}

impl MessageDecoder<MessagePrivate> for MessagePrivate {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> MessagePrivate {
        return MessagePrivate::new(
            buf.g8s(),
            unsafe { buf.data.get_unchecked(buf.pos..buf.pos + buf.data.len() - 8).to_vec() }
        );
    }
}

// ----

#[wasm_bindgen]
pub struct MessagePublic {
    #[wasm_bindgen(readonly)]
    pub color: u8,
    #[wasm_bindgen(readonly)]
    pub effect: u8,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub input: Vec<u8>,
}

#[wasm_bindgen]
impl MessagePublic {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: u8,
        effect: u8,
        input: Vec<u8>,
    ) -> MessagePublic {
        return MessagePublic {
            color,
            effect,
            input,
        }
    }
}

impl MessageDecoder<MessagePublic> for MessagePublic {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> MessagePublic {
        return MessagePublic::new(
            buf.g1(),
            buf.g1(),
            unsafe { buf.data.get_unchecked(buf.pos..buf.pos + buf.data.len() - 2).to_vec() }
        );
    }
}

// ----

#[wasm_bindgen]
pub struct MoveClick {
    #[wasm_bindgen(readonly)]
    pub ctrl: bool,
    #[wasm_bindgen(readonly)]
    pub op: bool,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub path: Vec<u32>,
}

#[wasm_bindgen]
impl MoveClick {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ctrl: bool,
        op: bool,
        path: Vec<u32>,
    ) -> MoveClick {
        return MoveClick {
            ctrl,
            op,
            path,
        }
    }
}

impl MessageDecoder<MoveClick> for MoveClick {
    fn length() -> i32 {
        return -1;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> MoveClick {
        let ctrl: bool = buf.g1() == 1;
        let x: u16 = buf.g2();
        let z: u16 = buf.g2();

        let offset: usize = if prot == ClientProt::MOVE_MINIMAPCLICK { 14 } else { 0 };
        let waypoints: usize = ((buf.data.len() - buf.pos - offset) / 2) + 1;

        let mut path: Vec<u32> = vec![0; waypoints];
        path[0] = CoordGrid::from(x, 0, z).packed;

        for index in 1..waypoints {
            if index >= 25 {
                break;
            }
            unsafe { *path.as_mut_ptr().add(index) = CoordGrid::from(x + buf.g1s() as u16, 0, z + buf.g1s() as u16).packed };
        }

        return MoveClick::new(
            ctrl,
            prot == ClientProt::MOVE_OPCLICK,
            path,
        );
    }
}

// ----

#[wasm_bindgen]
pub struct NoTimeout {}

#[wasm_bindgen]
impl NoTimeout {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NoTimeout {
        return NoTimeout {};
    }
}

impl MessageDecoder<NoTimeout> for NoTimeout {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> NoTimeout {
        return NoTimeout::new();
    }
}

// ----

#[wasm_bindgen]
pub struct OpHeld {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl OpHeld {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        obj: u16,
        slot: u16,
        component: u16,
    ) -> OpHeld {
        return OpHeld {
            op,
            obj,
            slot,
            component,
        }
    }
}

impl MessageDecoder<OpHeld> for OpHeld {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpHeld {
        let op: u8 = match prot {
            ClientProt::OPHELD1 => 1,
            ClientProt::OPHELD2 => 2,
            ClientProt::OPHELD3 => 3,
            ClientProt::OPHELD4 => 4,
            ClientProt::OPHELD5 => 5,
            _ => 0,
        };
        return OpHeld::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpHeldT {
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpHeldT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        obj: u16,
        slot: u16,
        component: u16,
        spell: u16,
    ) -> OpHeldT {
        return OpHeldT {
            obj,
            slot,
            component,
            spell,
        }
    }
}

impl MessageDecoder<OpHeldT> for OpHeldT {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpHeldT {
        return OpHeldT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpHeldU {
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub slot: u16,
    #[wasm_bindgen(readonly)]
    pub component: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpHeldU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        obj: u16,
        slot: u16,
        component: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpHeldU {
        return OpHeldU {
            obj,
            slot,
            component,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpHeldU> for OpHeldU {
    fn length() -> i32 {
        return 12;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpHeldU {
        return OpHeldU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpLoc {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
}

#[wasm_bindgen]
impl OpLoc {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        x: u16,
        z: u16,
        loc: u16,
    ) -> OpLoc {
        return OpLoc {
            op,
            x,
            z,
            loc,
        }
    }
}

impl MessageDecoder<OpLoc> for OpLoc {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpLoc {
        let op: u8 = match prot {
            ClientProt::OPLOC1 => 1,
            ClientProt::OPLOC2 => 2,
            ClientProt::OPLOC3 => 3,
            ClientProt::OPLOC4 => 4,
            ClientProt::OPLOC5 => 5,
            _ => 0,
        };
        return OpLoc::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpLocT {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpLocT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        loc: u16,
        spell: u16,
    ) -> OpLocT {
        return OpLocT {
            x,
            z,
            loc,
            spell,
        }
    }
}

impl MessageDecoder<OpLocT> for OpLocT {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpLocT {
        return OpLocT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpLocU {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub loc: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpLocU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        loc: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpLocU {
        return OpLocU {
            x,
            z,
            loc,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpLocU> for OpLocU {
    fn length() -> i32 {
        return 12;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpLocU {
        return OpLocU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpNpc {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub nid: u16,
}

#[wasm_bindgen]
impl OpNpc {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        nid: u16,
    ) -> OpNpc {
        return OpNpc {
            op,
            nid,
        }
    }
}

impl MessageDecoder<OpNpc> for OpNpc {
    fn length() -> i32 {
        return 2;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpNpc {
        let op: u8 = match prot {
            ClientProt::OPNPC1 => 1,
            ClientProt::OPNPC2 => 2,
            ClientProt::OPNPC3 => 3,
            ClientProt::OPNPC4 => 4,
            ClientProt::OPNPC5 => 5,
            _ => 0,
        };
        return OpNpc::new(
            op,
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpNpcT {
    #[wasm_bindgen(readonly)]
    pub nid: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpNpcT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        nid: u16,
        spell: u16,
    ) -> OpNpcT {
        return OpNpcT {
            nid,
            spell,
        }
    }
}

impl MessageDecoder<OpNpcT> for OpNpcT {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpNpcT {
        return OpNpcT::new(
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpNpcU {
    #[wasm_bindgen(readonly)]
    pub nid: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpNpcU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        nid: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpNpcU {
        return OpNpcU {
            nid,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpNpcU> for OpNpcU {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpNpcU {
        return OpNpcU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpObj {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
}

#[wasm_bindgen]
impl OpObj {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        x: u16,
        z: u16,
        obj: u16,
    ) -> OpObj {
        return OpObj {
            op,
            x,
            z,
            obj,
        }
    }
}

impl MessageDecoder<OpObj> for OpObj {
    fn length() -> i32 {
        return 6;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpObj {
        let op: u8 = match prot {
            ClientProt::OPOBJ1 => 1,
            ClientProt::OPOBJ2 => 2,
            ClientProt::OPOBJ3 => 3,
            ClientProt::OPOBJ4 => 4,
            ClientProt::OPOBJ5 => 5,
            _ => 0,
        };
        return OpObj::new(
            op,
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpObjT {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpObjT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        obj: u16,
        spell: u16,
    ) -> OpObjT {
        return OpObjT {
            x,
            z,
            obj,
            spell,
        }
    }
}

impl MessageDecoder<OpObjT> for OpObjT {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpObjT {
        return OpObjT::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpObjU {
    #[wasm_bindgen(readonly)]
    pub x: u16,
    #[wasm_bindgen(readonly)]
    pub z: u16,
    #[wasm_bindgen(readonly)]
    pub obj: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpObjU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: u16,
        z: u16,
        obj: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpObjU {
        return OpObjU {
            x,
            z,
            obj,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpObjU> for OpObjU {
    fn length() -> i32 {
        return 12;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpObjU {
        return OpObjU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpPlayer {
    #[wasm_bindgen(readonly)]
    pub op: u8,
    #[wasm_bindgen(readonly)]
    pub pid: u16,
}

#[wasm_bindgen]
impl OpPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        op: u8,
        pid: u16,
    ) -> OpPlayer {
        return OpPlayer {
            op,
            pid,
        }
    }
}

impl MessageDecoder<OpPlayer> for OpPlayer {
    fn length() -> i32 {
        return 2;
    }

    fn decode(prot: ClientProt, buf: &mut Packet) -> OpPlayer {
        let op: u8 = match prot {
            ClientProt::OPPLAYER1 => 1,
            ClientProt::OPPLAYER2 => 2,
            ClientProt::OPPLAYER3 => 3,
            ClientProt::OPPLAYER4 => 4,
            _ => 0,
        };
        return OpPlayer::new(
            op,
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpPlayerT {
    #[wasm_bindgen(readonly)]
    pub pid: u16,
    #[wasm_bindgen(readonly)]
    pub spell: u16,
}

#[wasm_bindgen]
impl OpPlayerT {
    #[wasm_bindgen(constructor)]
    pub fn new(
        pid: u16,
        spell: u16,
    ) -> OpPlayerT {
        return OpPlayerT {
            pid,
            spell,
        }
    }
}

impl MessageDecoder<OpPlayerT> for OpPlayerT {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpPlayerT {
        return OpPlayerT::new(
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct OpPlayerU {
    #[wasm_bindgen(readonly)]
    pub pid: u16,
    #[wasm_bindgen(readonly, js_name = useObj)]
    pub use_obj: u16,
    #[wasm_bindgen(readonly, js_name = useSlot)]
    pub use_slot: u16,
    #[wasm_bindgen(readonly, js_name = useComponent)]
    pub use_component: u16,
}

#[wasm_bindgen]
impl OpPlayerU {
    #[wasm_bindgen(constructor)]
    pub fn new(
        pid: u16,
        use_obj: u16,
        use_slot: u16,
        use_component: u16,
    ) -> OpPlayerU {
        return OpPlayerU {
            pid,
            use_obj,
            use_slot,
            use_component,
        }
    }
}

impl MessageDecoder<OpPlayerU> for OpPlayerU {
    fn length() -> i32 {
        return 8;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> OpPlayerU {
        return OpPlayerU::new(
            buf.g2(),
            buf.g2(),
            buf.g2(),
            buf.g2(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct RebuildGetMaps {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub maps: Vec<u32>,
}

#[wasm_bindgen]
impl RebuildGetMaps {
    #[wasm_bindgen(constructor)]
    pub fn new(maps: Vec<u32>) -> RebuildGetMaps {
        return RebuildGetMaps {
            maps,
        }
    }
}

impl MessageDecoder<RebuildGetMaps> for RebuildGetMaps {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> RebuildGetMaps {
        let mut maps: Vec<u32> = vec![0; buf.data.len() / 3];
        for index in 0..maps.len() {
            unsafe { *maps.as_mut_ptr().add(index) = buf.g3() as u32 };
        }
        return RebuildGetMaps::new(maps);
    }
}

// ----

#[wasm_bindgen]
pub struct ResumePauseButton {
    #[wasm_bindgen(readonly)]
    pub component: u16,
}

#[wasm_bindgen]
impl ResumePauseButton {
    pub fn new(component: u16) -> ResumePauseButton {
        return ResumePauseButton {
            component,
        };
    }
}

impl MessageDecoder<ResumePauseButton> for ResumePauseButton {
    fn length() -> i32 {
        return 2;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ResumePauseButton {
        return ResumePauseButton::new(buf.g2());
    }
}

// ----

#[wasm_bindgen]
pub struct ResumePCountDialog {
    #[wasm_bindgen(readonly)]
    pub input: i32,
}

#[wasm_bindgen]
impl ResumePCountDialog {
    #[wasm_bindgen(constructor)]
    pub fn new(input: i32) -> ResumePCountDialog {
        return ResumePCountDialog {
            input,
        }
    }
}

impl MessageDecoder<ResumePCountDialog> for ResumePCountDialog {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ResumePCountDialog {
        return ResumePCountDialog::new(buf.g4s());
    }
}

// ----

#[wasm_bindgen]
pub struct TutorialClickSide {
    #[wasm_bindgen(readonly)]
    pub tab: u8,
}

#[wasm_bindgen]
impl TutorialClickSide {
    #[wasm_bindgen(constructor)]
    pub fn new(tab: u8) -> TutorialClickSide {
        return TutorialClickSide {
            tab,
        }
    }
}

impl MessageDecoder<TutorialClickSide> for TutorialClickSide {
    fn length() -> i32 {
        return 1;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> TutorialClickSide {
        return TutorialClickSide::new(buf.g1());
    }
}

// ----

#[wasm_bindgen]
pub struct ChatSetMode {
    #[wasm_bindgen(readonly)]
    pub public: u8,
    #[wasm_bindgen(readonly)]
    pub private: u8,
    #[wasm_bindgen(readonly)]
    pub trade: u8,
}

#[wasm_bindgen]
impl ChatSetMode {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public: u8,
        private: u8,
        trade: u8,
    ) -> ChatSetMode {
        return ChatSetMode {
            public,
            private,
            trade,
        }
    }
}

impl MessageDecoder<ChatSetMode> for ChatSetMode {
    fn length() -> i32 {
        return 3;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ChatSetMode {
        return ChatSetMode::new(
            buf.g1(),
            buf.g1(),
            buf.g1(),
        );
    }
}

// ----

#[wasm_bindgen]
pub struct EventTracking {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub bytes: Vec<u8>,
}

#[wasm_bindgen]
impl EventTracking {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> EventTracking {
        return EventTracking {
            bytes,
        }
    }
}

impl MessageDecoder<EventTracking> for EventTracking {
    fn length() -> i32 {
        return -2;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> EventTracking {
        return EventTracking::new(buf.data.clone());
    }
}

// ----

#[wasm_bindgen]
pub struct ReportAbuse {
    #[wasm_bindgen(readonly)]
    pub offender: i64,
    #[wasm_bindgen(readonly)]
    pub reason: u8,
    #[wasm_bindgen(readonly)]
    pub mute: bool,
}

#[wasm_bindgen]
impl ReportAbuse {
    pub fn new(
        offender: i64,
        reason: u8,
        mute: bool,
    ) -> ReportAbuse {
        return ReportAbuse {
            offender,
            reason,
            mute,
        }
    }
}

impl MessageDecoder<ReportAbuse> for ReportAbuse {
    fn length() -> i32 {
        return 10;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> ReportAbuse {
        return ReportAbuse::new(
            buf.g8s(),
            buf.g1(),
            buf.g1() == 1,
        );
    }
}

// ----

#[wasm_bindgen]
pub struct EventCameraPosition {
    #[wasm_bindgen(readonly)]
    pub pitch: i32,
    #[wasm_bindgen(readonly)]
    pub yaw: i32,
    #[wasm_bindgen(readonly)]
    pub angle: i32,
    #[wasm_bindgen(readonly)]
    pub zoom: i32,
}

#[wasm_bindgen]
impl EventCameraPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(
        pitch: i32,
        yaw: i32,
        angle: i32,
        zoom: i32,
    ) -> EventCameraPosition {
        return EventCameraPosition {
            pitch,
            yaw,
            angle,
            zoom,
        }
    }
}

impl MessageDecoder<EventCameraPosition> for EventCameraPosition {
    fn length() -> i32 {
        return 6;
    }

    fn decode(_: ClientProt, buf: &mut Packet) -> EventCameraPosition {
        return EventCameraPosition::new(
            buf.g2() as i32,
            buf.g2() as i32,
            buf.g1() as i32,
            buf.g1() as i32,
        );
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp1 {}

#[wasm_bindgen]
impl AnticheatOp1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp1 {
        return AnticheatOp1 {};
    }
}

impl MessageDecoder<AnticheatOp1> for AnticheatOp1 {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp1 {
        return AnticheatOp1::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp2 {}

#[wasm_bindgen]
impl AnticheatOp2 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp2 {
        return AnticheatOp2 {};
    }
}

impl MessageDecoder<AnticheatOp2> for AnticheatOp2 {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp2 {
        return AnticheatOp2::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp3 {}

#[wasm_bindgen]
impl AnticheatOp3 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp3 {
        return AnticheatOp3 {};
    }
}

impl MessageDecoder<AnticheatOp3> for AnticheatOp3 {
    fn length() -> i32 {
        return 3;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp3 {
        return AnticheatOp3::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp4 {}

#[wasm_bindgen]
impl AnticheatOp4 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp4 {
        return AnticheatOp4 {};
    }
}

impl MessageDecoder<AnticheatOp4> for AnticheatOp4 {
    fn length() -> i32 {
        return 2;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp4 {
        return AnticheatOp4::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp5 {}

#[wasm_bindgen]
impl AnticheatOp5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp5 {
        return AnticheatOp5 {};
    }
}

impl MessageDecoder<AnticheatOp5> for AnticheatOp5 {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp5 {
        return AnticheatOp5::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp6 {}

#[wasm_bindgen]
impl AnticheatOp6 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp6 {
        return AnticheatOp6 {};
    }
}

impl MessageDecoder<AnticheatOp6> for AnticheatOp6 {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp6 {
        return AnticheatOp6::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp7 {}

#[wasm_bindgen]
impl AnticheatOp7 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp7 {
        return AnticheatOp7 {};
    }
}

impl MessageDecoder<AnticheatOp7> for AnticheatOp7 {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp7 {
        return AnticheatOp7::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp8 {}

#[wasm_bindgen]
impl AnticheatOp8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp8 {
        return AnticheatOp8 {};
    }
}

impl MessageDecoder<AnticheatOp8> for AnticheatOp8 {
    fn length() -> i32 {
        return 2;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp8 {
        return AnticheatOp8::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatOp9 {}

#[wasm_bindgen]
impl AnticheatOp9 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatOp9 {
        return AnticheatOp9 {};
    }
}

impl MessageDecoder<AnticheatOp9> for AnticheatOp9 {
    fn length() -> i32 {
        return 1;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatOp9 {
        return AnticheatOp9::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle1 {}

#[wasm_bindgen]
impl AnticheatCycle1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle1 {
        return AnticheatCycle1 {};
    }
}

impl MessageDecoder<AnticheatCycle1> for AnticheatCycle1 {
    fn length() -> i32 {
        return 1;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle1 {
        return AnticheatCycle1::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle2 {}

#[wasm_bindgen]
impl AnticheatCycle2 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle2 {
        return AnticheatCycle2 {};
    }
}

impl MessageDecoder<AnticheatCycle2> for AnticheatCycle2 {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle2 {
        return AnticheatCycle2::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle3 {}

#[wasm_bindgen]
impl AnticheatCycle3 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle3 {
        return AnticheatCycle3 {};
    }
}

impl MessageDecoder<AnticheatCycle3> for AnticheatCycle3 {
    fn length() -> i32 {
        return 3;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle3 {
        return AnticheatCycle3::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle4 {}

#[wasm_bindgen]
impl AnticheatCycle4 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle4 {
        return AnticheatCycle4 {};
    }
}

impl MessageDecoder<AnticheatCycle4> for AnticheatCycle4 {
    fn length() -> i32 {
        return 4;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle4 {
        return AnticheatCycle4::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle5 {}

#[wasm_bindgen]
impl AnticheatCycle5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle5 {
        return AnticheatCycle5 {};
    }
}

impl MessageDecoder<AnticheatCycle5> for AnticheatCycle5 {
    fn length() -> i32 {
        return 0;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle5 {
        return AnticheatCycle5::new();
    }
}

// ----

#[wasm_bindgen]
pub struct AnticheatCycle6 {}

#[wasm_bindgen]
impl AnticheatCycle6 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnticheatCycle6 {
        return AnticheatCycle6 {};
    }
}

impl MessageDecoder<AnticheatCycle6> for AnticheatCycle6 {
    fn length() -> i32 {
        return -1;
    }

    fn decode(_: ClientProt, _: &mut Packet) -> AnticheatCycle6 {
        return AnticheatCycle6::new();
    }
}