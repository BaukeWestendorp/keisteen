use uuid::Uuid;

use std::io::{self};

use crate::error::CraftError;
use crate::types::{Identifier, Position, VarInt};

mod config;
mod handshaking;
mod login;
mod play;
mod status;

pub use config::*;
pub use handshaking::*;
pub use login::*;
pub use play::*;
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

    pub fn consume_bool(&mut self) -> Result<bool, CraftError> {
        match self.consume_u8()? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            byte => Err(CraftError::InvalidBool(byte)),
        }
    }

    pub fn write_bool(&mut self, bool: bool) {
        match bool {
            false => self.bytes.push(0x00),
            true => self.bytes.push(0x01),
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn consume_u8(&mut self) -> Result<u8, CraftError> {
        if self.bytes.is_empty() {
            return Err(CraftError::UnexpectedEof);
        }

        let value = self.bytes.remove(0);
        Ok(value)
    }

    pub fn consume_i8(&mut self) -> Result<i8, CraftError> {
        let byte = self.consume_u8()?;
        Ok(byte as i8)
    }

    pub fn write_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    pub fn consume_u16(&mut self) -> Result<u16, CraftError> {
        if self.bytes.len() < 2 {
            return Err(CraftError::UnexpectedEof);
        }

        let value = u16::from_be_bytes([self.bytes[0], self.bytes[1]]);

        self.bytes.drain(0..2);

        Ok(value)
    }

    pub fn write_i32(&mut self, value: i32) {
        self.bytes.extend(value.to_be_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.bytes.extend(value.to_be_bytes());
    }

    pub fn consume_i64(&mut self) -> Result<i64, CraftError> {
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

    pub fn consume_identifier(&mut self) -> Result<Identifier, CraftError> {
        let string = self.consume_string(32767)?;
        string.parse()
    }

    pub fn write_identifier(&mut self, identifier: &Identifier) {
        self.write_string(&identifier.to_string(), 32767);
    }

    pub fn write_varint(&mut self, value: VarInt) {
        self.bytes.extend(value.to_bytes());
    }

    pub fn consume_varint(&mut self) -> Result<VarInt, CraftError> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = VarInt::from_reader(&mut cursor)?;
        self.bytes.drain(0..cursor.position() as usize);
        Ok(result)
    }

    pub fn write_position(&mut self, position: Position) {
        self.write_i64(position.into());
    }

    pub fn write_string(&mut self, value: &str, max_len: usize) {
        let truncated_value = if value.len() > max_len {
            let end_index = value.char_indices().nth(max_len).map_or(value.len(), |(idx, _)| idx);
            &value[..end_index]
        } else {
            value
        };
        let len = VarInt::new(truncated_value.len() as i32);
        self.write_varint(len);
        self.bytes.extend(truncated_value.as_bytes());
    }

    pub fn consume_string(&mut self, max_len: usize) -> Result<String, CraftError> {
        let length = self.consume_varint()?.raw() as usize;

        if length > max_len {
            return Err(CraftError::StringTooLong);
        }

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

    pub fn write_uuid(&mut self, uuid: Uuid) {
        self.bytes.extend(uuid.as_bytes());
    }

    pub fn consume_uuid(&mut self) -> Result<Uuid, CraftError> {
        const LENGTH: usize = 16;

        let Some(uuid_bytes) = self.bytes.last_chunk::<LENGTH>() else {
            return Err(CraftError::UnexpectedEof);
        };

        let uuid = Uuid::from_bytes(*uuid_bytes);

        Ok(uuid)
    }

    pub fn write_byte_array(&mut self, bytes: Vec<u8>) {
        self.bytes.extend(bytes);
    }

    pub fn write_identifier_array(&mut self, identifiers: Vec<Identifier>) {
        for identifier in identifiers {
            self.write_identifier(&identifier);
        }
    }

    pub fn consume_byte_array(&mut self, len: usize) -> Result<Vec<u8>, CraftError> {
        if self.bytes.len() < len {
            return Err(CraftError::UnexpectedEof);
        };

        let bytes = self.bytes.drain(0..len).collect();

        Ok(bytes)
    }

    pub fn write_prefixed_byte_array(&mut self, bytes: Vec<u8>) {
        self.write_varint(VarInt::new(bytes.len() as i32));
        self.write_byte_array(bytes);
    }

    pub fn consume_prefixed_byte_array(&mut self) -> Result<Vec<u8>, CraftError> {
        let len = self.consume_varint()?;
        let bytes = self.consume_byte_array(len.raw() as usize)?;
        Ok(bytes)
    }

    pub fn write_prefixed_identifier_array(&mut self, identifiers: Vec<Identifier>) {
        self.write_varint(VarInt::new(identifiers.len() as i32));
        self.write_identifier_array(identifiers);
    }
}

impl<T: Into<Vec<u8>>> From<T> for PacketData {
    fn from(bytes: T) -> Self {
        Self { bytes: bytes.into() }
    }
}
