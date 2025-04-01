use crate::packet::Packet;

pub struct WordPack {
    pack: Packet,
    unpack: Vec<char>,
}

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
    pub fn new() -> WordPack {
        return WordPack {
            pack: Packet::new(100),
            unpack: Vec::with_capacity(100),
        };
    }

    #[inline]
    pub unsafe fn unpack(&mut self, mut packet: Packet, length: usize) -> String {
        self.unpack.clear();

        let mut carry: i32 = -1;

        for _ in 0..length {
            if self.unpack.len() >= 80 {
                break;
            }

            let data: u8 = packet.g1();
            let mut nibble: u8 = (data >> 4) & 0xf;

            if carry != -1 {
                self.unpack.push(*WordPack::CHAR_LOOKUP.as_ptr().add((((carry << 4) + nibble as i32) - 195) as usize));
                carry = -1;
            } else if nibble < 13 {
                self.unpack.push(*WordPack::CHAR_LOOKUP.as_ptr().add(nibble as usize));
            } else {
                carry = nibble as i32;
            }

            nibble = data & 0xf;

            if carry != -1 {
                self.unpack.push(*WordPack::CHAR_LOOKUP.as_ptr().add((((carry << 4) + nibble as i32) - 195) as usize));
                carry = -1;
            } else if nibble < 13 {
                self.unpack.push(*WordPack::CHAR_LOOKUP.as_ptr().add(nibble as usize));
            } else {
                carry = nibble as i32;
            }
        }

        WordPack::sentence_case(&mut self.unpack);

        return self.unpack.iter().collect();
    }

    #[inline]
    pub unsafe fn pack(&mut self, input: String) -> Vec<u8> {
        self.pack.pos = 0;

        let mut carry: i32 = -1;
        let mut count: i32 = 0;

        for c in input.chars() {
            if count >= 80 {
                break;
            }

            let c: char = c.to_ascii_lowercase();
            count += 1;

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
                    self.pack.p1(index);
                }
            } else if index < 13 {
                self.pack.p1((carry << 4) + index);
                carry = -1;
            } else {
                self.pack.p1((carry << 4) + (index >> 4));
                carry = index & 0xf;
            }
        }

        if carry != -1 {
            self.pack.p1(carry << 4);
        }

        return self.pack.data.get_unchecked(0..self.pack.pos).to_vec();
    }

    #[inline]
    pub fn sentence_case(chars: &mut [char]) {
        let mut punctuation = true;

        for c in chars.iter_mut() {
            if punctuation && c.is_ascii_lowercase() {
                *c = c.to_ascii_uppercase();
                punctuation = false;
            }

            if *c == '.' || *c == '!' {
                punctuation = true;
            }
        }
    }
}