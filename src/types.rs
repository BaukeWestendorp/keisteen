use std::{fmt, io, ops};

use crate::error::CraftError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTextComponent {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn new(namespace: String, value: String) -> Result<Self, CraftError> {
        // Validate namespace
        if !namespace.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-' || c == '_'
        }) {
            return Err(CraftError::InvalidIdentifierNamespace(namespace));
        }

        // Validate value
        if !value.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '.'
                || c == '-'
                || c == '_'
                || c == '/'
        }) {
            return Err(CraftError::InvalidIdentifierValue(value));
        }

        Ok(Self { namespace, value })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt(i32);

impl VarInt {
    pub const SEGMENT_BITS: u8 = 0x7F;
    pub const CONTINUE_BIT: u8 = 0x80;

    pub fn new(raw: i32) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> i32 {
        self.0
    }

    pub fn len(&self) -> usize {
        let mut n = self.0 as u64;
        let mut len = 1;
        while n >= Self::CONTINUE_BIT as u64 {
            n >>= 7;
            len += 1;
        }
        len
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            if (self.0 & !Self::SEGMENT_BITS as i32) == 0 {
                bytes.push(self.0 as u8);
                break;
            }

            bytes.push(((self.0 & Self::SEGMENT_BITS as i32) | Self::CONTINUE_BIT as i32) as u8);

            self.0 = ((self.0 as u32) >> 7) as i32;
        }
        bytes
    }

    pub fn from_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut value = 0;
        let mut position = 0;
        let mut current_byte;

        loop {
            let mut byte_buf = [0];
            reader.read(&mut byte_buf)?;
            current_byte = byte_buf[0];

            value |= ((current_byte & Self::SEGMENT_BITS) as i32) << position;

            if (current_byte & Self::CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt is too big"));
            }
        }

        Ok(Self::new(value))
    }

    pub fn to_writer<W: io::Write>(self, writer: &mut W) -> io::Result<()> {
        let bytes = self.to_bytes();
        writer.write_all(&bytes)
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
