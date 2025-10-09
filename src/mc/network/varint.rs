use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt(i32);

impl VarInt {
    pub const SEGMENT_BITS: u8 = 0x7F;
    pub const CONTINUE_BIT: u8 = 0x80;

    pub const fn new(raw: i32) -> Self {
        Self(raw)
    }

    pub const fn raw(&self) -> i32 {
        self.0
    }

    pub const fn byte_count(&self) -> usize {
        let mut n = self.0 as u64;
        let mut len = 1;
        while n >= Self::CONTINUE_BIT as u64 {
            n >>= 7;
            len += 1;
        }
        len
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut n = self.raw();
        let mut bytes = Vec::new();
        while (n & !0x7F) != 0 {
            bytes.push(((n & 0x7F) | 0x80) as u8);
            n = ((n as u32) >> 7) as i32;
        }
        bytes.push(n as u8);
        bytes
    }
}

impl ops::Add for VarInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl ops::Sub for VarInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl ops::Mul for VarInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl ops::Div for VarInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl fmt::Display for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
