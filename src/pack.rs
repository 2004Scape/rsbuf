use crate::packet::Packet;

pub struct WordPack;

impl WordPack {
    const CHAR_LOOKUP: [char; 61] = [
        ' ',
        'e', 't', 'a', 'o', 'i', 'h', 'n', 's', 'r', 'd', 'l', 'u', 'm',
        'w', 'c', 'y', 'f', 'g', 'p', 'b', 'v', 'k', 'x', 'j', 'q', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ' ', '!', '?', '.', ',', ':', ';', '(', ')', '-',
        '&', '*', '\\', '\'', '@', '#', '+', '=', '£', '$', '%', '"', '[', ']'
    ];

    #[inline]
    pub unsafe fn unpack(packet: &mut Packet, length: usize) -> String {
        let mut char_buffer: Vec<char> = Vec::with_capacity(80);
        let mut pos: usize = 0;
        let mut carry: i32 = -1;

        for _ in 0..length {
            if pos >= 80 {
                break;
            }

            let data: u8 = packet.g1();
            let mut nibble: u8 = (data >> 4) & 0xf;

            if carry != -1 {
                let index: i32 = ((carry << 4) + nibble as i32) - 195;
                if index >= 0 && index < WordPack::CHAR_LOOKUP.len() as i32 {
                    *char_buffer.as_mut_ptr().add(pos) = *WordPack::CHAR_LOOKUP.as_ptr().add(index as usize);
                    pos += 1;
                }
                carry = -1;
            } else if nibble < 13 {
                *char_buffer.as_mut_ptr().add(pos) = *WordPack::CHAR_LOOKUP.as_ptr().add(nibble as usize);
                pos += 1;
            } else {
                carry = nibble as i32;
            }

            nibble = data & 0xf;

            if carry != -1 {
                let index: i32 = ((carry << 4) + nibble as i32) - 195;
                if index >= 0 && index < WordPack::CHAR_LOOKUP.len() as i32 {
                    *char_buffer.as_mut_ptr().add(pos) = *WordPack::CHAR_LOOKUP.as_ptr().add(index as usize);
                    pos += 1;
                }
                carry = -1;
            } else if nibble < 13 {
                *char_buffer.as_mut_ptr().add(pos) = *WordPack::CHAR_LOOKUP.as_ptr().add(nibble as usize);
                pos += 1;
            } else {
                carry = nibble as i32;
            }
        }

        WordPack::sentence_case(&char_buffer.get_unchecked(..pos).iter().collect::<String>())
    }

    #[inline]
    pub unsafe fn pack(packet: &mut Packet, mut input: String) {
        if input.len() > 80 {
            input.truncate(80);
        }
        input = input.to_lowercase();
        let mut carry: i32 = -1;

        for c in input.chars() {
            let mut index = 0;
            for (j, &ch) in WordPack::CHAR_LOOKUP.iter().enumerate() {
                if ch == c {
                    index = j as i32;
                    break;
                }
            }

            if index > 12 {
                index += 195;
            }

            if carry == -1 {
                if index < 13 {
                    carry = index;
                } else {
                    packet.p1(index);
                }
            } else if index < 13 {
                packet.p1((carry << 4) + index);
                carry = -1;
            } else {
                packet.p1((carry << 4) + (index >> 4));
                carry = index & 0xf;
            }
        }

        if carry != -1 {
            packet.p1(carry << 4);
        }
    }

    #[inline]
    pub fn sentence_case(input: &str) -> String {
        let mut chars: Vec<char> = input.to_lowercase().chars().collect();
        let mut punctuation: bool = true;

        for c in chars.iter_mut() {
            if punctuation && c.is_ascii_lowercase() {
                *c = c.to_ascii_uppercase();
                punctuation = false;
            }

            if *c == '.' || *c == '!' {
                punctuation = true;
            }
        }

        return chars.iter().collect();
    }
}