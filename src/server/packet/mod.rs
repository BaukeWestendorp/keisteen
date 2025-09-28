use std::io::{self};

use crate::error::CraftError;
use crate::types::VarInt;

mod handshaking;
mod status;

pub use handshaking::*;
pub use status::*;

#[derive(Debug)]
pub struct RawPacket {
    pub packet_id: VarInt,
    pub data: PacketData,
}

impl RawPacket {
    pub fn length(&self) -> usize {
        self.data.bytes().len() + self.packet_id.len()
    }
}

#[derive(Debug)]
pub struct PacketData {
    bytes: Vec<u8>,
}

impl PacketData {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn write_long(&mut self, value: i64) {
        self.bytes.extend(value.to_be_bytes());
    }

    pub fn consume_long(&mut self) -> Result<i64, CraftError> {
        if self.bytes.len() < 8 {
            return Err(CraftError::UnexpectedEof);
        }

        let value = i64::from_be_bytes([
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
            self.bytes[6],
            self.bytes[7],
        ]);

        self.bytes.drain(0..8);

        Ok(value)
    }

    pub fn write_var_int(&mut self, value: VarInt) {
        self.bytes.extend(value.to_bytes());
    }

    pub fn consume_var_int(&mut self) -> Result<VarInt, CraftError> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = VarInt::from_reader(&mut cursor)?;
        self.bytes.drain(0..cursor.position() as usize);
        Ok(result)
    }

    pub fn write_string(&mut self, value: String) {
        let len = VarInt::new(value.len() as i32);
        self.write_var_int(len);
        self.bytes.extend(value.as_bytes());
    }

    pub fn consume_string(&mut self) -> Result<String, CraftError> {
        let length = self.consume_var_int()?.raw() as usize;

        if length > 32767 * 3 {
            return Err(CraftError::StringTooLong);
        }

        if self.bytes.len() < length {
            return Err(CraftError::UnexpectedEof);
        }

        let string_bytes = self.bytes.drain(0..length).collect::<Vec<u8>>();
        let string = String::from_utf8(string_bytes).map_err(|_| CraftError::InvalidUtf8)?;

        Ok(string)
    }

    pub fn consume_u16(&mut self) -> Result<u16, CraftError> {
        if self.bytes.len() < 2 {
            return Err(CraftError::UnexpectedEof);
        }

        let value = u16::from_be_bytes([self.bytes[0], self.bytes[1]]);

        self.bytes.drain(0..2);

        Ok(value)
    }
}

impl<T: Into<Vec<u8>>> From<T> for PacketData {
    fn from(bytes: T) -> Self {
        Self { bytes: bytes.into() }
    }
}
