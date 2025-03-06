#[repr(u16)]
#[derive(Eq, Hash, PartialEq)]
pub enum PlayerInfoProt {
    Appearance = 0x1,
    Anim = 0x2,
    FaceEntity = 0x4,
    Say = 0x8,
    Damage = 0x10,
    FaceCoord = 0x20,
    Chat = 0x40,
    Big = 0x80,
    SpotAnim = 0x100,
    ExactMove = 0x200,
}