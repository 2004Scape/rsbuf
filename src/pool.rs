use crate::packet::Packet;

pub struct PacketPool {
    packet_100b: Packet,
    packet_5kb: Packet,
}

impl PacketPool {
    pub fn new() -> PacketPool {
        return PacketPool {
            packet_100b: Packet::new(100),
            packet_5kb: Packet::new(5000),
        }
    }

    pub fn take(&mut self, len: usize) -> &mut Packet {
        return if len <= 100 {
            let packet: &mut Packet = &mut self.packet_100b;
            packet.pos = 0;
            packet.bit_pos = 0;
            packet
        } else {
            let packet: &mut Packet = &mut self.packet_5kb;
            packet.pos = 0;
            packet.bit_pos = 0;
            packet
        }
    }
}