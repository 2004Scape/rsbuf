use std::io::Error;
use num_bigint::BigInt;
use num_traits::identities::One;
use pem::{parse, Pem};
use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};

#[derive(Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub pos: usize,
    pub bit_pos: usize,
}

impl Packet {
    /// Create a new `Packet` with a fixed sized allocated buffer.
    /// This `Packet` will default to use byte mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rsbuf::packet::Packet;
    ///
    /// let packet: Packet = Packet::new(5);
    /// ```
    #[inline]
    pub fn new(len: usize) -> Packet {
        return Packet {
            data: vec![0; len],
            pos: 0,
            bit_pos: 0,
        };
    }

    /// Create a new `Packet` from a `Vec<u8> array.`
    /// This will take ownership of the input vector.
    /// This `Packet` will default to use byte mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rsbuf::packet::Packet;
    ///
    /// let packet: Packet = Packet::from(vec![0; 128]);
    /// ```
    pub fn from(data: Vec<u8>) -> Packet {
        return Packet {
            data,
            pos: 0,
            bit_pos: 0,
        };
    }

    /// Create a new `Packet` from an input file from IO.
    /// This `Packet` will default to use byte mode.
    pub fn io(path: String) -> Result<Packet, Error> {
        return match std::fs::read(path) {
            Ok(bytes) => Ok(Packet::from(bytes)),
            Err(err) => Err(err),
        };
    }

    /// Returns the remaining amount of storage available for this `Packet`.
    /// This is calculated by the difference of the total length with the current
    /// position of this packet.
    /// (`self.len() - self.pos`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rsbuf::packet::Packet;
    ///
    /// let mut packet: Packet = Packet::new(5);
    /// packet.pos += 1;
    /// assert_eq!(4, packet.remaining());
    /// ```
    #[inline]
    pub fn remaining(&self) -> i32 {
        return (self.len() - self.pos) as i32;
    }

    /// Returns the total usize (length) of this `Packet` allocated for storage of bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rsbuf::packet::Packet;
    ///
    /// let mut packet: Packet = Packet::new(1);
    /// assert_eq!(1, packet.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    /// Writes the lower 8 bits of the given `value` into the internal buffer
    /// at the current position, and increments the byte position of this buffer by 1.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. Only the lower 8 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 1 byte
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 1 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there is at least 1 byte available as required by the function's safety
    /// contract.
    #[inline]
    pub fn p1(&mut self, value: i32) {
        unsafe { *self.data.get_unchecked_mut(self.pos) = value as u8 }
        self.pos += 1;
    }

    /// Writes the lower 16 bits of the given `value` into the internal buffer
    /// at the current position, and increments the byte position of this buffer by 2.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. Only the lower 16 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 2 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 2 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn p2(&mut self, value: i32) {
        let start: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(start..start + 2) }
            .copy_from_slice(&(value as u16).to_be_bytes());
        self.pos += 2;
    }

    /// Writes the lower 16 bits of the given `value` into the internal buffer in little-endian
    /// at the current position, and increments the byte position of this buffer by 2.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. Only the lower 16 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 2 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 2 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn ip2(&mut self, value: i32) {
        let start: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(start..start + 2) }
            .copy_from_slice(&(value as u16).to_le_bytes());
        self.pos += 2;
    }

    /// Writes the lower 24 bits of the given `value` into the internal buffer
    /// at the current position, and increments the byte position of this buffer by 3.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. Only the lower 24 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 3 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 3 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn p3(&mut self, value: i32) {
        let start: usize = self.pos;
        unsafe { *self.data.get_unchecked_mut(start) = (value >> 16) as u8 };
        unsafe { self.data.get_unchecked_mut(start + 1..start + 3) }
            .copy_from_slice(&(value as u16).to_be_bytes());
        self.pos += 3;
    }

    /// Writes the 32 bits of the given `value` into the internal buffer
    /// at the current position, and increments the byte position of this buffer by 4.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. All 32 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 4 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 4 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn p4(&mut self, value: i32) {
        let start: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(start..start + 4) }
            .copy_from_slice(&value.to_be_bytes());
        self.pos += 4;
    }

    /// Writes the 32 bits of the given `value` into the internal buffer in little-endian
    /// at the current position, and increments the byte position of this buffer by 4.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i32` integer. All 32 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 4 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 4 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn ip4(&mut self, value: i32) {
        let start: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(start..start + 4) }
            .copy_from_slice(&value.to_le_bytes());
        self.pos += 4;
    }

    /// Writes the 64 bits of the given `value` into the internal buffer
    /// at the current position, and increments the byte position of this buffer by 8.
    ///
    /// # Arguments
    ///
    /// * `value` - An `i64` long integer. All 64 bits are used for storage.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to write to the internal buffer without bounds checking.
    /// The caller must ensure that there is sufficient space in the buffer (`self.data`) for 8 bytes
    /// at the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by 8 after writing to the buffer.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn p8(&mut self, value: i64) {
        let start: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(start..start + 8) }
            .copy_from_slice(&value.to_be_bytes());
        self.pos += 8;
    }

    /// Writes the provided string to the internal buffer, followed by a terminator byte.
    ///
    /// # Arguments
    ///
    /// * `str` - The string to be written to the buffer.
    /// * `terminator` - The byte value (`u8`) that marks the end of the string.
    ///
    /// # Safety
    ///
    /// This function assumes that (`self.pos`) is valid and that there is enough space in (`self.data`)
    /// to write the entire string and terminator. The caller must ensure that the buffer is large enough
    /// to accommodate the string plus the terminator byte to avoid out-of-bounds access.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by the length of the string plus the terminator.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked_mut()` fails. This should never happen if
    /// the caller ensures that there are at least string length + terminator bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn pjstr(&mut self, str: &str, terminator: u8) {
        let mut length: usize = self.pos;
        for byte in str.bytes() {
            unsafe { *self.data.get_unchecked_mut(length) = byte };
            length += 1;
        }
        unsafe { *self.data.get_unchecked_mut(length) = terminator };
        self.pos = length + 1;
    }

    /// Writes an integer value to the internal buffer based on its range, encoding it into one or two bytes.
    ///
    /// The function first checks if the value is in the range of 0 to 127 (inclusive). If so, it writes the value as a single byte
    /// using the `p1` method. If the value is in the range of 0 to 32767 (exclusive), the function adjusts the value by adding
    /// `32768`, then writes it as two bytes using the `p2` method. If the value is outside these ranges, it triggers a panic.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to be written to the internal buffer. It must be in one of the supported ranges:
    ///   - 0 <= value < 128 (handled by `p1`)
    ///   - 0 <= value < 32768 (handled by `p2` after adjusting by `+32768`)
    ///
    /// # Side Effects
    ///
    /// This function will modify the internal buffer, writing the value as either 1 or 2 bytes based on the range of the value.
    /// The current position in the buffer will be updated accordingly.
    ///
    /// # Panics
    ///
    /// This function will panic if the `value` is outside the supported ranges (i.e., not in the interval [0, 127]
    /// or [0, 32767]). See (g1) and (g2) for additional panics from this function.
    #[inline]
    pub fn psmart(&mut self, value: i32) {
        if value >= 0 && value < 128 {
            self.p1(value);
        } else if value >= 0 && value < 32768 {
            self.p2(value + 32768);
        } else {
            panic!("Error psmart out of range: {}", value);
        }
    }

    /// Writes a signed integer value to the internal buffer, encoding it into one or two bytes based on its range.
    ///
    /// The function first checks if the value is in the range of -64 to 63 (inclusive). If so, it writes the value adjusted
    /// by adding `64` as a single byte using the `p1` method. If the value is in the range of -16384 to 16383 (exclusive),
    /// the function adjusts the value by adding `49152`, then writes it as two bytes using the `p2` method. If the value is
    /// outside these ranges, it triggers a panic.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed integer value to be written to the internal buffer. It must be in one of the supported ranges:
    ///   - -64 <= value <= 63 (handled by `p1` after adjusting by `+64`)
    ///   - -16384 <= value <= 16383 (handled by `p2` after adjusting by `+49152`)
    ///
    /// # Side Effects
    ///
    /// This function will modify the internal buffer, writing the signed value as either 1 or 2 bytes based on the value's range.
    /// The current position in the buffer will be updated accordingly.
    ///
    /// # Panics
    ///
    /// This function will panic if the `value` is outside the supported ranges (i.e., not in the interval [-64, 63]
    /// or [-16384, 16383]). See (g1) and (g2) for additional panics from this function.
    #[inline]
    pub fn psmarts(&mut self, value: i32) {
        if value < 64 && value >= -64 {
            self.p1(value + 64);
        } else if value < 16384 && value >= -16384 {
            self.p2(value + 49152);
        } else {
            panic!("Error psmarts out of range: {}", value);
        }
    }

    /// Copies a slice of bytes from the provided source (`src`) into the internal buffer at the current position.
    /// The function writes a specified range of bytes from the source into the buffer and then updates the position
    /// in the buffer to reflect the number of bytes written.
    ///
    /// The function works by slicing the `src` array and copying the specified range into the internal data buffer.
    /// The slice is determined by the `offset` and `length` arguments, where `offset` is the starting index in the
    /// source array, and `length` is the number of bytes to copy. The position (`pos`) is incremented by the number
    /// of bytes copied after the operation.
    ///
    /// # Arguments
    ///
    /// * `src` - A slice of bytes (`&[u8]`) representing the source data to be copied into the internal buffer.
    /// * `offset` - The starting index in the `src` slice from where the copy should begin.
    /// * `length` - The number of bytes to copy from the `src` slice starting at the given `offset`.
    ///
    /// # Safety
    ///
    /// This function uses unsafe code for performance reasons, leveraging `get_unchecked_mut` and `get_unchecked` to
    /// avoid bounds checking. It's the caller's responsibility to ensure that the `offset` and `length` values are valid,
    /// i.e., that they do not exceed the bounds of the `src` slice or the internal buffer.
    ///
    /// # Side Effects
    ///
    /// This function modifies the internal buffer (`self.data`), copying the specified range of bytes into it.
    /// The position (`self.pos`) is updated by the number of bytes copied (`end`), so the next write operation will
    /// begin at the correct position in the buffer.
    ///
    /// # Panics
    ///
    /// This function will panic if the `offset` and `length` values do not fit within the bounds of the `src` slice,
    /// or if the number of bytes to be copied exceeds the capacity of the internal buffer.
    #[inline]
    pub fn pdata(&mut self, src: &Vec<u8>, offset: usize, length: usize) {
        let pos: usize = self.pos;
        unsafe { self.data.get_unchecked_mut(pos..pos + length) }
            .copy_from_slice(unsafe { &src.get_unchecked(offset..offset + length) });
        self.pos += length;
    }

    /// Reads one byte from the internal buffer, interprets them as an unsigned 8-bit integer (`u8`),
    /// and increments the internal position by 1.
    ///
    /// # Return
    ///
    /// Returns an `u8` value created from one byte starting at the current position.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there is at least 1 byte remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 1 after reading the byte.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there is at least 1 byte available as required by the function's safety
    /// contract.
    #[inline]
    pub fn g1(&mut self) -> u8 {
        self.pos += 1;
        return unsafe { *self.data.get_unchecked(self.pos - 1) };
    }

    /// Reads one byte from the internal buffer, interprets them as a signed 8-bit integer (`i8`),
    /// and increments the internal position by 1.
    ///
    /// # Return
    ///
    /// Returns an `i8` value created from one byte starting at the current position.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there is at least 1 byte remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 1 after reading the byte.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there is at least 1 byte available as required by the function's safety
    /// contract.
    #[inline]
    pub fn g1s(&mut self) -> i8 {
        self.pos += 1;
        return (unsafe { *self.data.get_unchecked(self.pos - 1) }) as i8;
    }

    /// Reads two bytes from the internal buffer, interprets them as a big-endian unsigned 16-bit integer (`u16`),
    /// and increments the internal position by 2.
    ///
    /// # Return
    ///
    /// Returns an `u16` value created from the two bytes starting at the current position in big-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 2 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 2 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn g2(&mut self) -> u16 {
        self.pos += 2;
        let pos: usize = self.pos;
        return u16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads two bytes from the internal buffer, interprets them as a big-endian signed 16-bit integer (`i16`),
    /// and increments the internal position by 2.
    ///
    /// # Return
    ///
    /// Returns an `i16` value created from the two bytes starting at the current position in big-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 2 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 2 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn g2s(&mut self) -> i16 {
        self.pos += 2;
        let pos: usize = self.pos;
        return i16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads two bytes from the internal buffer, interprets them as a little-endian signed 16-bit integer (`i16`),
    /// and increments the internal position by 2.
    ///
    /// # Return
    ///
    /// Returns an `i16` value created from the two bytes starting at the current position in little-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 2 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 2 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 2 bytes available as required by the function's safety
    /// contract.
    #[inline]
    pub fn ig2s(&mut self) -> i16 {
        self.pos += 2;
        let pos: usize = self.pos;
        return i16::from_le_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads three bytes from the internal buffer, interprets them as a big-endian unsigned (24-bit) 32-bit integer (`i32`),
    /// and increments the internal position by 3.
    ///
    /// # Return
    ///
    /// Returns an `i32` value created from the three bytes starting at the current position in big-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 3 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 3 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 3 bytes available as required by the function's safety
    /// contract.
    // java ints are always signed (java 8 added unsigned)
    #[inline]
    pub fn g3(&mut self) -> i32 {
        self.pos += 3;
        let pos: usize = self.pos;
        return ((unsafe { *self.data.get_unchecked(pos - 3) } as u32) << 16
            | u16::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 2..pos) }
                .try_into()
                .unwrap(),
        ) as u32) as i32;
    }

    /// Reads four bytes from the internal buffer, interprets them as a big-endian signed 32-bit integer (`i32`),
    /// and increments the internal position by 4.
    ///
    /// # Return
    ///
    /// Returns an `i32` value created from the four bytes starting at the current position in big-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 4 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 4 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 4 bytes available as required by the function's safety
    /// contract.
    // java ints are always signed (java 8 added unsigned)
    #[inline]
    pub fn g4s(&mut self) -> i32 {
        self.pos += 4;
        let pos: usize = self.pos;
        return i32::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 4..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads four bytes from the internal buffer, interprets them as a little-endian signed 32-bit integer (`i32`),
    /// and increments the internal position by 4.
    ///
    /// # Return
    ///
    /// Returns an `i32` value created from the four bytes starting at the current position in little-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 4 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 4 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 4 bytes available as required by the function's safety
    /// contract.
    // java ints are always signed (java 8 added unsigned)
    #[inline]
    pub fn ig4s(&mut self) -> i32 {
        self.pos += 4;
        let pos: usize = self.pos;
        return i32::from_le_bytes(
            unsafe { self.data.get_unchecked(pos - 4..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads eight bytes from the internal buffer, interprets them as a big-endian signed 64-bit long integer (`i64`),
    /// and increments the internal position by 8.
    ///
    /// # Return
    ///
    /// Returns an `i64` value created from the eight bytes starting at the current position in big-endian format.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there are at least 8 bytes remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. Failure to ensure this may result
    /// in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by 8 after reading the bytes.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least 8 bytes available as required by the function's safety
    /// contract.
    // java longs are always signed (java 8 added unsigned)
    #[inline]
    pub fn g8s(&mut self) -> i64 {
        self.pos += 8;
        let pos: usize = self.pos;
        return i64::from_be_bytes(
            unsafe { self.data.get_unchecked(pos - 8..pos) }
                .try_into()
                .unwrap(),
        );
    }

    /// Reads a string from the internal buffer until a terminator byte is encountered.
    ///
    /// # Arguments
    ///
    /// * `terminator` - The byte (`u8`) value that marks the end of the string.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the bytes read from the buffer, excluding the terminator.
    ///
    /// # Safety
    ///
    /// This function assumes that `self.pos` is valid and that there is enough data remaining in
    /// `self.data` to read. The caller must ensure that bounds are respected to avoid out-of-bounds
    /// access.
    ///
    /// # Side Effects
    ///
    /// The function increments the position (`self.pos`) by the length of the string plus the terminator.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` or `from_utf8_unchecked()` fails. This should never happen if
    /// the caller ensures that there are at least string length + terminator bytes available as required by the function's safety
    /// contract. The caller must also ensure that the backing buffer vector will decode the string with valid UTF-8 characters.
    #[inline]
    pub fn gjstr(&mut self, terminator: u8) -> String {
        let pos: usize = self.pos;
        let mut length = pos;
        while unsafe { *self.data.get_unchecked(length) } != terminator {
            length += 1;
        }
        let str: &str =
            unsafe { std::str::from_utf8_unchecked(&self.data.get_unchecked(pos..length)) };
        self.pos = length + 1;
        return str.to_owned();
    }

    /// Reads one byte from the internal buffer and interprets it to decide whether to read one or two bytes.
    /// If the byte is less than `128`, it reads a single byte as a `u8` and returns it as a signed `i32`.
    /// If the byte is `128` or greater, it reads two bytes, interprets them as an unsigned 16-bit integer (`u16`),
    /// and subtracts `32768` from the result, returning the result as a signed `i32` where the value captures
    /// the lower bits as an unsigned value.
    ///
    /// # Return
    ///
    /// Returns an `i32` value. The value is either the result of reading a single byte (if it's less than `128`),
    /// or the result of reading two bytes with `32768` subtracted (if the byte is `128` or greater).
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there is at least 1 byte remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. If the byte is `128` or greater,
    /// it will attempt to read two additional bytes. The caller must ensure that there are at least 2 bytes
    /// available from `self.pos`. Failure to ensure this may result in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by either 1 or 2 depending on the byte value.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails, which would typically
    /// happen if there are insufficient bytes in the buffer as required by the function's safety contract.
    /// The caller must ensure there are at least 2 bytes available from `self.pos` before calling this function.
    /// See (g1) and (g2) for additional panics from this function.
    #[inline]
    pub fn gsmart(&mut self) -> i32 {
        return if unsafe { *self.data.get_unchecked(self.pos) } < 128 {
            self.g1() as i32
        } else {
            self.g2() as i32 - 32768
        };
    }

    /// Reads one byte from the internal buffer and interprets it to decide whether to read one or two bytes.
    /// If the byte is less than `128`, it reads a single byte as a `u8` and subtracts `64` from the result,
    /// returning it as a signed i32.
    /// If the byte is `128` or greater, it reads two bytes, interprets them as an unsigned 16-bit integer (`u16`),
    /// and subtracts `49152` from the result, returning the result as a signed `i32` where the value captures
    /// the lower bits as a signed value.
    ///
    /// # Return
    ///
    /// Returns an `i32` value. The value is either the result of reading a single byte (if it's less than `128`),
    /// subtracted by `64`, or the result of reading two bytes with `49152` subtracted (if the byte is `128` or greater).
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to read from the internal buffer without bounds checking.
    /// It is the caller's responsibility to ensure that there is at least 1 byte remaining in the buffer
    /// from the current position (`self.pos`) before calling this function. If the byte is `128` or greater,
    /// it will attempt to read two additional bytes. The caller must ensure that there are at least 2 bytes
    /// available from `self.pos`. Failure to ensure this may result in undefined behavior.
    ///
    /// # Side Effects
    ///
    /// The function increments the internal position (`self.pos`) by either 1 or 2 depending on the byte value.
    ///
    /// # Panics
    ///
    /// The function will panic if the slice conversion via `get_unchecked()` fails, which would typically
    /// happen if there are insufficient bytes in the buffer as required by the function's safety contract.
    /// The caller must ensure there are at least 2 bytes available from `self.pos` before calling this function.
    /// See (g1) and (g2) for additional panics from this function.
    #[inline]
    pub fn gsmarts(&mut self) -> i32 {
        return if unsafe { *self.data.get_unchecked(self.pos) } < 128 {
            self.g1() as i32 - 64
        } else {
            self.g2() as i32 - 49152
        };
    }

    /// Copies a slice of bytes from the internal buffer into the provided destination slice (`dest`).
    /// The function reads a specified range of bytes from the internal buffer and copies them into the destination.
    /// After the operation, the position (`pos`) in the internal buffer is updated to reflect the number of bytes
    /// that have been read.
    ///
    /// The function uses the `offset` and `length` arguments to determine the range of bytes to copy. The `offset`
    /// is the starting index in the destination slice, and `length` is the number of bytes to copy. The position (`pos`)
    /// is incremented by the number of bytes read from the internal buffer.
    ///
    /// # Arguments
    ///
    /// * `dest` - A mutable slice of bytes (`&mut [u8]`) representing the destination to copy data into.
    /// * `offset` - The starting index in the `dest` slice where the copy should begin.
    /// * `length` - The number of bytes to copy from the internal buffer to the `dest` slice starting at the given `offset`.
    ///
    /// # Safety
    ///
    /// This function uses unsafe code for performance reasons, leveraging `get_unchecked_mut` and `get_unchecked` to
    /// avoid bounds checking. It is the caller's responsibility to ensure that the `offset` and `length` values are valid,
    /// i.e., that they do not exceed the bounds of the `dest` slice or the internal buffer (`self.data`).
    ///
    /// # Side Effects
    ///
    /// This function modifies the `dest` slice, copying the specified range of bytes from the internal buffer into it.
    /// The position (`self.pos`) in the internal buffer is updated by the number of bytes copied, so the next read operation
    /// will begin at the correct position in the buffer.
    ///
    /// # Panics
    ///
    /// This function will panic if the `offset` and `length` values do not fit within the bounds of the destination slice
    /// or if the number of bytes to be copied exceeds the size of the internal buffer.
    #[inline]
    pub fn gdata(&mut self, dest: &mut Vec<u8>, offset: usize, length: usize) {
        let pos: usize = self.pos;
        unsafe { dest.get_unchecked_mut(offset..offset + length) }
            .copy_from_slice(unsafe { &self.data.get_unchecked(pos..pos + length) });
        self.pos += length;
    }

    /// Sets the internal bit position (`bit_pos`) to the current byte position (`pos`)
    /// converted to bit position. This is typically used when switching from byte-based
    /// to bit-based addressing.
    ///
    /// # Side Effects
    ///
    /// The function updates the internal `bit_pos` to the current byte position (`self.pos`)
    /// multiplied by 8 (`byte pos << 3`), effectively aligning it to the corresponding bit position.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    /// However, please note to only use bit functions while in bits mode. You must call the `bytes`
    /// function in order to switch back to byte mode to use byte functions.
    #[inline]
    pub fn bits(&mut self) {
        self.bit_pos = self.pos << 3;
    }

    /// Sets the internal byte position (`pos`) based on the current bit position (`bit_pos`).
    /// This is typically used when switching from bit-based addressing back to byte-based addressing.
    ///
    /// The byte position is calculated by dividing the current bit position by 8 and rounding down.
    ///
    /// # Side Effects
    ///
    /// The function updates the internal `pos` to reflect the byte index corresponding to the current
    /// bit position (`self.bit_pos`). This is done by adding 7 to `self.bit_pos` and performing a right shift by 3.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    /// However, please note to only use byte functions while in bytes mode. You must call the `bits`
    /// function in order to switch back to bit mode to use bit functions.
    #[inline]
    pub fn bytes(&mut self) {
        self.pos = (self.bit_pos + 7) >> 3;
    }

    /// Reads a specified number of bits from the internal buffer, starting from the current bit position (`self.bit_pos`).
    /// The function updates the internal bit position (`self.bit_pos`) after reading the bits and returns the value of
    /// the extracted bits as a `u8`. It handles cases where the bits span across multiple bytes and adjusts accordingly.
    ///  It also calculates the bitmasks needed on the fly.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits to read from the internal buffer starting at the current bit position (`self.bit_pos`).
    ///
    /// # Description
    ///
    /// The function begins by determining the current bit position (`self.bit_pos`) and increases it by the number of bits
    /// (`n`) to ensure the bit position is updated after the operation. It then reads the specified number of bits from the
    /// internal buffer (`self.data`), accounting for cases where the bits may span multiple bytes. The function carefully
    /// calculates the required bit shifts and masks to correctly extract the desired bits.
    ///
    /// The process is as follows:
    /// - If the bits to be read span across more than one byte, the function handles each byte separately, reading and
    ///   shifting the bits into place until the entire value is extracted.
    /// - If all bits fit within a single byte, the function extracts them from the current byte using bitwise shifts and
    ///   masks, and then updates the internal bit position.
    ///
    /// # Side Effects
    ///
    /// - The internal bit position (`self.bit_pos`) is incremented by `n`, ensuring the next bit operation starts at the
    ///   correct position in the buffer.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The bit position (`self.bit_pos`) or the number of bits to read (`n`) causes out-of-bounds memory access in
    ///   `self.data`.
    /// - `n` exceeds the available bits in the current sequence or span of bytes.
    pub fn gbit(&mut self, mut n: usize) -> i32 {
        let pos: usize = self.bit_pos;
        self.bit_pos += n;

        let mut byte_pos: usize = pos >> 3;
        let mut remaining: usize = 8 - (pos & 7);

        let mut result: u8 = 0;

        while n > remaining {
            result |= (unsafe { *self.data.as_ptr().add(byte_pos) } & (1 << remaining) - 1)
                << (n - remaining);
            byte_pos += 1;
            n -= remaining;
            remaining = 8;
        }

        if n == remaining {
            result |= unsafe { *self.data.as_ptr().add(byte_pos) } & (1 << remaining) - 1;
        } else {
            result |=
                (unsafe { *self.data.as_ptr().add(byte_pos) } >> (remaining - n)) & (1 << n) - 1;
        }
        return result as i32;
    }

    /// Writes a specified number of bits to the internal buffer, starting from the current bit position (`self.bit_pos`).
    /// The function updates the internal bit position (`self.bit_pos`) after writing the bits, and modifies the data
    /// in `self.data` by setting the corresponding bits based on the value provided (`val`). It also calculates
    /// the bitmasks needed on the fly.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits to write. The function writes `n` bits from the provided `val` starting at the current
    ///   bit position (`self.bit_pos`).
    /// * `val` - A `u8` value from which the bits are extracted and written to the internal buffer. The value will be
    ///   shifted and masked to fit the specified number of bits (`n`).
    ///
    /// # Description
    ///
    /// The function starts by determining the current bit position (`self.bit_pos`) and adjusts it by the number of bits
    /// (`n`) to update the position for future bit writes. It then writes the bits of `val` into `self.data` starting at
    /// the current bit position, taking care to manage byte boundaries and handling cases where the bits span across
    /// multiple bytes. The function ensures that no overflow or underflow occurs when shifting and masking the bits.
    ///
    /// It works as follows:
    /// - If the bits to be written span across multiple bytes, the function processes each byte individually, updating
    ///   the data and continuing until all `n` bits have been written.
    /// - If all the bits can fit within a single byte, the function writes them directly to the current byte and updates
    ///   the internal bit position.
    ///
    /// # Side Effects
    ///
    /// - The internal bit position (`self.bit_pos`) is updated by `n` after the write, ensuring that the next bit
    ///   operation will continue from the correct position in the buffer.
    /// - The internal buffer (`self.data`) is modified by setting the corresponding bits at the current position.
    ///
    /// # Panics
    ///
    /// This function will panic if the bit position (`self.bit_pos`) or the number of bits (`n`) leads to accessing
    /// out-of-bounds memory in `self.data` or if `n` exceeds the number of available bits in the current byte/sequence.
    pub fn pbit(&mut self, mut n: usize, val: i32) {
        let pos: usize = self.bit_pos;
        self.bit_pos += n;

        let mut byte_pos: usize = pos >> 3;
        let mut remaining: usize = 8 - (pos & 7);

        while n > remaining {
            let shift: i32 = ((1 << remaining) - 1) as i32;
            let byte: i32 = unsafe { *self.data.as_ptr().add(byte_pos) } as i32;
            unsafe {
                *self.data.as_mut_ptr().add(byte_pos) =
                    ((byte & !shift) | ((val >> (n - remaining)) & shift)) as u8
            }
            byte_pos += 1;
            n -= remaining;
            remaining = 8;
        }

        let r: i32 = (remaining - n) as i32;
        let shift: i32 = (1 << n) - 1;
        let byte: i32 = unsafe { *self.data.as_ptr().add(byte_pos) } as i32;
        unsafe {
            *self.data.as_mut_ptr().add(byte_pos) = ((byte & (!shift << r)) | ((val & shift) << r)) as u8
        }
    }

    /// Writes a single byte (`u8`) value at a calculated position in the internal buffer.
    /// The position is determined by subtracting the given `size` and 1 from the current position (`self.pos`).
    ///
    /// # Arguments
    ///
    /// * `size` - A `u8` value to be written into the internal buffer. It represents the size of data to store at the position.
    ///
    /// # Description
    ///
    /// The function calculates the position by subtracting `size + 1` from the current buffer position (`self.pos`) and writes
    /// the `size` byte at that location. It uses an unsafe block to directly modify the internal buffer without bounds checking,
    /// assuming the caller guarantees safe indexing.
    ///
    /// # Side Effects
    ///
    /// The internal buffer (`self.data`) is modified by writing the `size` value at the calculated position.
    ///
    /// # Panics
    ///
    /// This function may panic if the computed position (`self.pos - size - 1`) leads to accessing out-of-bounds memory in `self.data`.
    #[inline]
    pub fn psize1(&mut self, size: u8) {
        unsafe { *self.data.get_unchecked_mut(self.pos - size as usize - 1) = size };
    }

    /// Writes a 2-byte (`u16`) value at a calculated position in the internal buffer.
    /// The position is determined by subtracting the given `size` and 2 from the current position (`self.pos`).
    ///
    /// # Arguments
    ///
    /// * `size` - A `u16` value to be written into the internal buffer. It represents the size of data to store at the position.
    ///
    /// # Description
    ///
    /// The function calculates the position by subtracting `size + 2` from the current buffer position (`self.pos`) and writes
    /// the `size` as two bytes (big-endian order) at that location. It uses `to_be_bytes` to convert the `u16` value to its
    /// big-endian byte representation. An unsafe block is used for unchecked memory access to improve performance.
    ///
    /// # Side Effects
    ///
    /// The internal buffer (`self.data`) is modified by writing the `size` value as two bytes at the calculated position.
    ///
    /// # Panics
    ///
    /// This function may panic if the computed position (`self.pos - size - 2`) leads to accessing out-of-bounds memory in `self.data`.
    #[inline]
    pub fn psize2(&mut self, size: u16) {
        let pos: usize = self.pos - size as usize - 2;
        unsafe { self.data.get_unchecked_mut(pos..pos + 2) }.copy_from_slice(&size.to_be_bytes());
    }

    /// Writes a 4-byte (`u32`) value at a calculated position in the internal buffer.
    /// The position is determined by subtracting the given `size` and 4 from the current position (`self.pos`).
    ///
    /// # Arguments
    ///
    /// * `size` - A `u32` value to be written into the internal buffer. It represents the size of data to store at the position.
    ///
    /// # Description
    ///
    /// The function calculates the position by subtracting `size + 4` from the current buffer position (`self.pos`) and writes
    /// the `size` as four bytes (big-endian order) at that location. It uses `to_be_bytes` to convert the `u32` value to its
    /// big-endian byte representation. An unsafe block is used for unchecked memory access to improve performance.
    ///
    /// # Side Effects
    ///
    /// The internal buffer (`self.data`) is modified by writing the `size` value as four bytes at the calculated position.
    ///
    /// # Panics
    ///
    /// This function may panic if the computed position (`self.pos - size - 4`) leads to accessing out-of-bounds memory in `self.data`.
    #[inline]
    pub fn psize4(&mut self, size: u32) {
        let pos: usize = self.pos - size as usize - 4;
        unsafe { self.data.get_unchecked_mut(pos..pos + 4) }.copy_from_slice(&size.to_be_bytes());
    }

    /// Performs RSA encryption using the provided PEM-encoded public key.
    ///
    /// # Arguments
    ///
    /// * `pem_key`: A string slice containing the PEM-encoded RSA private key.
    ///              This key is used to extract the public key components (`n` and `e`).
    ///
    /// # Description
    ///
    /// This function encrypts the data in the internal buffer using RSA encryption with the provided
    /// public key components (`n` and `e`) derived from the PEM key. It follows these steps:
    ///
    /// 1. Parses the PEM string and extracts the RSA public key modulus (`n`) and exponent (`e`).
    /// 2. Reads the data from the buffer into a temporary vector.
    /// 3. Converts the data into a `BigInt` and performs modular exponentiation (modPow) using `n` and `e`.
    /// 4. Converts the result back into bytes and writes the encrypted data back into the buffer.
    ///
    /// # Workflow:
    ///
    /// - Data is read from the internal buffer and prepared for encryption.
    /// - The encryption is carried out using the public key modulus and exponent.
    /// - The encrypted data is written back to the buffer.
    pub fn rsaenc(&mut self, pem_key: &str) {
        let pem: Pem = parse(pem_key).expect("Failed to parse PEM key");
        let private_key: RsaPrivateKey =
            RsaPrivateKey::from_pkcs8_der(&pem.contents()).expect("Failed to decode private key");

        let n: BigInt =
            BigInt::from_bytes_be(num_bigint::Sign::Plus, &private_key.n().to_bytes_be());
        let e: BigInt =
            BigInt::from_bytes_be(num_bigint::Sign::Plus, &private_key.e().to_bytes_be());

        let length: usize = self.pos;
        self.pos = 0;
        let mut dec: Vec<u8> = vec![0u8; length];
        self.gdata(&mut dec, 0, length);

        let big_raw: BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus, &dec);
        let big_enc: BigInt = big_raw.modpow(&e, &n);
        let enc: Vec<u8> = big_enc.to_bytes_be().1;

        self.pos = 0;
        self.p1(enc.len() as i32);
        self.pdata(&enc, 0, enc.len());
    }

    /// Performs RSA decryption using the Chinese Remainder Theorem (CRT) and the provided PEM-encoded private key.
    ///
    /// # Arguments
    ///
    /// * `pem_key`: A string slice containing the PEM-encoded RSA private key.
    ///              The key is used to extract the RSA CRT components: `p`, `q`, `dP`, `dQ`, and `qInv`.
    ///
    /// # Description
    ///
    /// This function decrypts the data in the internal buffer using the RSA decryption process and the CRT method.
    /// The CRT optimizes the decryption process by using the private key components derived from the PEM key:
    /// - `p`, `q`: The prime factors of the RSA modulus.
    /// - `dP`, `dQ`: The private exponents modulo `p` and `q`.
    /// - `qInv`: The modular inverse of `q` modulo `p`.
    ///
    /// The decryption process follows these steps:
    ///
    /// 1. Parse the PEM key and extract the private key components: `p`, `q`, `dP`, `dQ`, and `qInv`.
    /// 2. Read the encrypted data from the buffer into a temporary vector.
    /// 3. Perform RSA decryption using modular exponentiation and the Chinese Remainder Theorem.
    /// 4. Reconstruct the original message using CRT and write the decrypted data back to the buffer.
    ///
    /// # Workflow:
    ///
    /// - Data is read from the buffer and prepared for decryption.
    /// - The decryption is carried out using CRT, which computes two partial results `m1` and `m2`.
    /// - The final result is reconstructed from `m1` and `m2`, and the decrypted data is written back to the buffer.
    pub fn rsadec(&mut self, pem_key: &str) {
        let pem: Pem = parse(pem_key).expect("Failed to parse PEM key");
        let private_key: RsaPrivateKey =
            RsaPrivateKey::from_pkcs8_der(&pem.contents()).expect("Failed to decode private key");

        let p: BigInt = BigInt::from_bytes_be(
            num_bigint::Sign::Plus,
            &private_key.primes()[0].to_bytes_be(),
        );
        let q: BigInt = BigInt::from_bytes_be(
            num_bigint::Sign::Plus,
            &private_key.primes()[1].to_bytes_be(),
        );
        let d_p: BigInt = BigInt::from_bytes_be(
            num_bigint::Sign::Plus,
            &private_key.dp().unwrap().to_bytes_be(),
        );
        let d_q: BigInt = BigInt::from_bytes_be(
            num_bigint::Sign::Plus,
            &private_key.dq().unwrap().to_bytes_be(),
        );
        let q_inv: BigInt = BigInt::from_bytes_be(
            num_bigint::Sign::Plus,
            &private_key.qinv().unwrap().to_bytes_be().1,
        );

        let length: usize = self.g1() as usize;
        let mut enc: Vec<u8> = vec![0u8; length];
        self.gdata(&mut enc, 0, length);

        let big_raw: BigInt = BigInt::from_bytes_be(num_bigint::Sign::Plus, &enc);
        let m1: BigInt = big_raw.modpow(&d_p, &p);
        let m2: BigInt = big_raw.modpow(&d_q, &q);
        let h: BigInt = (q_inv * (&m1 - &m2)).modpow(&BigInt::one(), &p);
        let dec: Vec<u8> = (&m2 + &h * &q).to_bytes_be().1;

        self.pos = 0;
        self.pdata(&dec, 0, dec.len());
        self.pos = 0;
    }
}