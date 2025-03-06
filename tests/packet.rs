use rsbuf::packet::Packet;

#[test]
fn test_p1() {
    let mut packet: Packet = Packet::new(1);
    packet.p1(127);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(127, packet.g1s());
}

#[test]
fn test_p2() {
    let mut packet: Packet = Packet::new(2);
    packet.p2(32767);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(32767, packet.g2s());
}

#[test]
fn test_ip2() {
    let mut packet: Packet = Packet::new(2);
    packet.ip2(32767);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(32767, packet.ig2s());
}

#[test]
fn test_p3() {
    let mut packet: Packet = Packet::new(3);
    packet.p3(16777215);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(16777215, packet.g3());
}

#[test]
fn test_p4() {
    let mut packet: Packet = Packet::new(4);
    packet.p4(2147483647);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(2147483647, packet.g4s());
}

#[test]
fn test_ip4() {
    let mut packet: Packet = Packet::new(4);
    packet.ip4(2147483647);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(2147483647, packet.ig4s());
}

#[test]
fn test_p8() {
    let mut packet: Packet = Packet::new(8);
    packet.p8(9223372036854775807);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(9223372036854775807, packet.g8s());
}

#[test]
fn test_pjstr() {
    let str: &str = "Hello World!";
    let mut packet: Packet = Packet::new(str.len() + 1);
    packet.pjstr(str, 0);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(str, packet.gjstr(0));
}

#[test]
fn test_psmart_1() {
    let mut packet: Packet = Packet::new(1);
    packet.psmart(69);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(69, packet.gsmart());
}

#[test]
fn test_psmart_2() {
    let mut packet: Packet = Packet::new(2);
    packet.psmart(3454);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(3454, packet.gsmart());
}

#[test]
fn test_psmarts_1() {
    let mut packet: Packet = Packet::new(1);
    packet.psmarts(-13);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(-13, packet.gsmarts());
}

#[test]
fn test_psmarts_2() {
    let mut packet: Packet = Packet::new(2);
    packet.psmarts(-3454);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(-3454, packet.gsmarts());
}

#[test]
fn test_pdata() {
    let mut packet: Packet = Packet::new(3);
    let src: Vec<u8> = vec![1, 2, 3, 4, 5];
    packet.pdata(&src, 1, 3); // Copies bytes 2, 3, and 4 from `src` into the packet's buffer
    assert_eq!(packet.data, vec![2, 3, 4]);
}

#[test]
fn test_g1() {
    let mut packet: Packet = Packet::new(1);
    packet.p1(255);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(255, packet.g1());
}

#[test]
fn test_g1s() {
    let mut packet: Packet = Packet::new(1);
    packet.p1(127);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(127, packet.g1s());
}

#[test]
fn test_g2() {
    let mut packet: Packet = Packet::new(2);
    packet.p2(65535);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(65535, packet.g2());
}

#[test]
fn test_g2s() {
    let mut packet: Packet = Packet::new(2);
    packet.p2(32767);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(32767, packet.g2s());
}

#[test]
fn test_ig2s() {
    let mut packet: Packet = Packet::new(2);
    packet.ip2(32767);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(32767, packet.ig2s());
}

#[test]
fn test_g3() {
    let mut packet: Packet = Packet::new(3);
    packet.p3(16777215);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(16777215, packet.g3());
}

#[test]
fn test_g4s() {
    let mut packet: Packet = Packet::new(4);
    packet.p4(2147483647);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(2147483647, packet.g4s());
}

#[test]
fn test_ig4s() {
    let mut packet: Packet = Packet::new(4);
    packet.ip4(2147483647);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(2147483647, packet.ig4s());
}

#[test]
fn test_g8s() {
    let mut packet: Packet = Packet::new(8);
    packet.p8(9223372036854775807);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(9223372036854775807, packet.g8s());
}

#[test]
fn test_gjstr() {
    let str: &str = "Hello World!";
    let mut packet: Packet = Packet::new(str.len() + 1);
    packet.pjstr(str, 0);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(str, packet.gjstr(0));
}

#[test]
fn test_gsmart_1() {
    let mut packet: Packet = Packet::new(1);
    packet.psmart(69);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(69, packet.gsmart());
}

#[test]
fn test_gsmart_2() {
    let mut packet: Packet = Packet::new(2);
    packet.psmart(3454);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(3454, packet.gsmart());
}

#[test]
fn test_gsmarts_1() {
    let mut packet: Packet = Packet::new(1);
    packet.psmarts(-13);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(-13, packet.gsmarts());
}

#[test]
fn test_gsmarts_2() {
    let mut packet: Packet = Packet::new(2);
    packet.psmarts(-3454);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(-3454, packet.gsmarts());
}

#[test]
fn test_gdata() {
    let mut packet: Packet = Packet::from(vec![10, 20, 30, 40, 50]);
    let mut dest: Vec<u8> = vec![0u8; 3]; // Create a destination slice with enough space to copy 3 bytes
    packet.gdata(&mut dest, 1, 2); // Copy the first 3 bytes from the internal buffer to `dest`
    assert_eq!(dest, vec![0, 10, 20]); // Verify the correct data was copied
}

#[test]
fn test_gbit() {
    let mut packet: Packet = Packet::new(2);
    packet.bits(); // Switch to bits mode.
    packet.pbit(1, 0);
    packet.pbit(4, 3);
    packet.pbit(7, 13);
    packet.bytes(); // Switch to bytes mode.
    packet.pos = 0; // Resetting the packet for showing test case.
    packet.bit_pos = 0; // Resetting the packet for showing test case.
    packet.bits(); // Switch to bits mode.
    assert_eq!(0, packet.gbit(1));
    assert_eq!(3, packet.gbit(4));
    assert_eq!(13, packet.gbit(7));
    packet.bytes(); // Switch to bytes mode.
}

#[test]
fn test_pbit() {
    let mut packet: Packet = Packet::new(2);
    packet.bits(); // Switch to bits mode.
    packet.pbit(1, 0);
    packet.pbit(4, 3);
    packet.pbit(7, 13);
    packet.bytes(); // Switch to bytes mode.
    packet.pos = 0; // Resetting the packet for showing test case.
    packet.bit_pos = 0; // Resetting the packet for showing test case.
    packet.bits(); // Switch to bits mode.
    assert_eq!(0, packet.gbit(1));
    assert_eq!(3, packet.gbit(4));
    assert_eq!(13, packet.gbit(7));
    packet.bytes(); // Switch to bytes mode.
}

#[test]
fn test_psize1() {
    let mut packet = Packet::new(2);
    packet.pos += 1;
    packet.p1(69);
    packet.psize1(1);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(1, packet.g1());
    assert_eq!(69, packet.g1());
}

#[test]
fn test_psize2() {
    let mut packet = Packet::new(4);
    packet.pos += 2;
    packet.p2(65535);
    packet.psize2(2);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(2, packet.g2());
    assert_eq!(65535, packet.g2());
}

#[test]
fn test_psize4() {
    let mut packet = Packet::new(8);
    packet.pos += 4;
    packet.p4(2147483647);
    packet.psize4(4);
    packet.pos = 0; // Resetting the packet for showing test case.
    assert_eq!(4, packet.g4s());
    assert_eq!(2147483647, packet.g4s());
}

#[test]
fn test_rsaenc() {
    let key = r#"-----BEGIN PRIVATE KEY-----
    MIIBcgIBADANBgkqhkiG9w0BAQEFAASCAVwwggFYAgEAAkEAiMOHSKWCKPcmHNw0
    C1aR19CXXe4OzbcXYJ5r+XHrP+cj750TDkaGgTc5dorZRy60bYv8wELBpfywXpMf
    Yy7qXQIhAIHzkLLPjKcDnuUHl1lR1aCxWoe/iz+ZyWaDQRjFD9lNAkBXH7BiBIth
    ch6/zx6HcVMkG3DDqibtsPnwahsr4HxOReq6T8NW6oBsvtKY04YTWQpT/eA4PDpB
    F1hRYpMkCSXlAiEA12bEXY1UEkjlCVg2WUOK14Ug0Kd8fKayvoFWOoUTahkCIQCi
    inQF+Sys+2JOzUx80OWHR/JqcF6eqc20u7PnfB1S5QIhAJIcknTm7h3OH3kbx5Dq
    AtzL3tEJyD83H3EMM8GRTmB9AiBb6wjlrcM3AIG08VSVyhxCTeUwS9ck5NaNV8LM
    LFx19QIgFdwct6Ho3H2nTDvthwudGhnE1rwbQEeTy9eOAMUMSSY=
    -----END PRIVATE KEY-----
    "#;
    let mut packet = Packet::new(65 + 1);
    packet.pjstr("hello", 0);
    packet.pjstr("world", 0);
    packet.rsaenc(key); // Uses modulus and exponent from the private key to encrypt (client).
    let mut result = Packet::from(packet.data);
    result.rsadec(key); // Uses CRT to decrypt (server).
    assert_eq!("hello", result.gjstr(0));
    assert_eq!("world", result.gjstr(0));
}

#[test]
fn test_rsadec() {
    let key = r#"-----BEGIN PRIVATE KEY-----
    MIIBcgIBADANBgkqhkiG9w0BAQEFAASCAVwwggFYAgEAAkEAiMOHSKWCKPcmHNw0
    C1aR19CXXe4OzbcXYJ5r+XHrP+cj750TDkaGgTc5dorZRy60bYv8wELBpfywXpMf
    Yy7qXQIhAIHzkLLPjKcDnuUHl1lR1aCxWoe/iz+ZyWaDQRjFD9lNAkBXH7BiBIth
    ch6/zx6HcVMkG3DDqibtsPnwahsr4HxOReq6T8NW6oBsvtKY04YTWQpT/eA4PDpB
    F1hRYpMkCSXlAiEA12bEXY1UEkjlCVg2WUOK14Ug0Kd8fKayvoFWOoUTahkCIQCi
    inQF+Sys+2JOzUx80OWHR/JqcF6eqc20u7PnfB1S5QIhAJIcknTm7h3OH3kbx5Dq
    AtzL3tEJyD83H3EMM8GRTmB9AiBb6wjlrcM3AIG08VSVyhxCTeUwS9ck5NaNV8LM
    LFx19QIgFdwct6Ho3H2nTDvthwudGhnE1rwbQEeTy9eOAMUMSSY=
    -----END PRIVATE KEY-----
    "#;
    let mut packet = Packet::new(65 + 1);
    packet.pjstr("hello", 0);
    packet.pjstr("world", 0);
    packet.rsaenc(key); // Uses modulus and exponent from the private key to encrypt (client).
    let mut result = Packet::from(packet.data);
    result.rsadec(key); // Uses CRT to decrypt (server).
    assert_eq!("hello", result.gjstr(0));
    assert_eq!("world", result.gjstr(0));
}