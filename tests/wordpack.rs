use rsbuf::packet::Packet;
use rsbuf::wordpack::WordPack;

#[test]
fn test_unpack_test() {
    let packet: Packet = Packet::from(vec![33, 130]);
    unsafe { assert_eq!("Test", WordPack::new().unpack(packet, 2)) };
}

#[test]
fn test_unpack_zezima() {
    let packet: Packet = Packet::from(vec![221, 29, 213, 208, 48]);
    unsafe { assert_eq!("Zezima ", WordPack::new().unpack(packet, 5)) };
}

#[test]
fn test_pack_test() {
    unsafe { assert_eq!(WordPack::new().pack("Test".to_string()), vec![33, 130]) };
}

#[test]
fn test_pack_zezima() {
    unsafe { assert_eq!(WordPack::new().pack("Zezima".to_string()), vec![221, 29, 213, 208, 48]) };
}
