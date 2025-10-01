use std::io;
use std::str::FromStr;

use crate::error::CraftError;
use crate::nbt::CompoundTag;
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
use uuid::Uuid;

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
    read_pos: usize,
}

impl PacketData {
    pub fn new() -> Self {
        Self { bytes: Vec::new(), read_pos: 0 }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn write_all<W: ProtocolWrite>(&mut self, value: W) {
        value.write_all(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }

    pub fn write_all_prefixed<W: PrefixedProtocolWrite>(&mut self, value: W) {
        value.prefixed_write_all(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }

    pub fn read<R: ProtocolRead>(&mut self) -> Result<R, CraftError> {
        let mut cursor = io::Cursor::new(&self.bytes[self.read_pos..]);
        let result = R::read_from(&mut cursor)?;
        self.read_pos += cursor.position() as usize;
        Ok(result)
    }

    pub fn read_prefixed<R: PrefixedProtocolRead>(&mut self) -> Result<R, CraftError> {
        let mut cursor = io::Cursor::new(&self.bytes[self.read_pos..]);
        let result = R::read_from_prefixed(&mut cursor)?;
        self.read_pos += cursor.position() as usize;
        Ok(result)
    }

    pub fn read_predefined<T, R: PredefinedProtocolRead<T>>(
        &mut self,
        info: T,
    ) -> Result<R, CraftError> {
        let mut cursor = io::Cursor::new(&self.bytes[self.read_pos..]);
        let result = R::read_from_predefined(&mut cursor, info)?;
        self.read_pos += cursor.position() as usize;
        Ok(result)
    }
}

impl<T: Into<Vec<u8>>> From<T> for PacketData {
    fn from(bytes: T) -> Self {
        Self { bytes: bytes.into(), read_pos: 0 }
    }
}

pub trait ProtocolWrite {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

pub trait PrefixedProtocolWrite: ProtocolWrite {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl ProtocolWrite for () {
    fn write_all<W: io::Write>(&self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl PrefixedProtocolWrite for () {
    fn prefixed_write_all<W: io::Write>(&self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl ProtocolWrite for bool {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            false => writer.write(&[0x00])?,
            true => writer.write(&[0x01])?,
        };
        Ok(())
    }
}

macro_rules! impl_protocol_write {
    ($type:ty) => {
        impl ProtocolWrite for $type {
            fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
                writer.write_all(&self.to_be_bytes())
            }
        }
    };
}

impl_protocol_write!(u8);
impl_protocol_write!(i8);
impl_protocol_write!(u16);
impl_protocol_write!(i16);
impl_protocol_write!(u32);
impl_protocol_write!(i32);
impl_protocol_write!(u64);
impl_protocol_write!(i64);

impl ProtocolWrite for String {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        VarInt::new(self.len() as i32).to_writer(writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl ProtocolWrite for Identifier {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.to_string().write_all(writer)
    }
}

impl ProtocolWrite for VarInt {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.to_writer(writer)
    }
}

impl ProtocolWrite for CompoundTag {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        todo!();
    }
}

impl ProtocolWrite for Position {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        i64::from(*self).write_all(writer)
    }
}

impl ProtocolWrite for Uuid {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.to_bytes_le())
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Option<T> {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Some(t) => t.write_all(writer),
            None => Ok(()),
        }
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Option<T> {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Some(t) => {
                true.write_all(writer)?;
                t.write_all(writer)?;
            }
            None => {
                false.write_all(writer)?;
            }
        }
        Ok(())
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Vec<T> {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        for item in self {
            item.write_all(writer)?;
        }
        Ok(())
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Vec<T> {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let len = VarInt::new(self.len() as i32);
        len.write_all(writer)?;
        self.write_all(writer)?;
        Ok(())
    }
}

pub trait ProtocolRead {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError>
    where
        Self: Sized;
}

pub trait PrefixedProtocolRead {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> Result<Self, CraftError>
    where
        Self: Sized;
}

pub trait PredefinedProtocolRead<T> {
    fn read_from_predefined<R: io::Read>(reader: &mut R, info: T) -> Result<Self, CraftError>
    where
        Self: Sized;
}

impl ProtocolRead for bool {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        match u8::read_from(reader)? {
            0x01 => Ok(true),
            0x00 => Ok(false),
            byte => Err(CraftError::InvalidBool(byte)),
        }
    }
}

macro_rules! impl_protocol_read {
    ($type:ty) => {
        impl ProtocolRead for $type {
            fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
                const SIZE: usize = (<$type>::BITS / 8) as usize;
                let mut buf = [0u8; SIZE];
                reader.read_exact(&mut buf)?;
                Ok(<$type>::from_be_bytes(buf))
            }
        }
    };
}

impl_protocol_read!(u8);
impl_protocol_read!(i8);
impl_protocol_read!(u16);
impl_protocol_read!(i16);
impl_protocol_read!(u32);
impl_protocol_read!(i32);
impl_protocol_read!(u64);
impl_protocol_read!(i64);

impl ProtocolRead for String {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        let length = VarInt::from_reader(reader)?.raw() as usize;

        let mut string_buf = vec![0u8; length];
        reader.read_exact(&mut string_buf)?;

        Self::from_utf8(string_buf).map_err(|_| CraftError::InvalidUtf8)
    }
}

impl ProtocolRead for Identifier {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        let string = String::read_from(reader)?;
        Self::from_str(&string)
    }
}

impl ProtocolRead for VarInt {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        Ok(Self::from_reader(reader)?)
    }
}

impl ProtocolRead for CompoundTag {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        todo!();
    }
}

impl ProtocolRead for Position {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        Ok(i64::read_from(reader)?.into())
    }
}

impl ProtocolRead for Uuid {
    fn read_from<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes)?;
        Ok(Self::from_bytes(bytes))
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Option<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        match bool::read_from(reader)? {
            true => Ok(Some(T::read_from(reader)?)),
            false => Ok(None),
        }
    }
}

impl<T: ProtocolRead> PredefinedProtocolRead<bool> for Option<T> {
    fn read_from_predefined<R: io::Read>(
        reader: &mut R,
        should_read: bool,
    ) -> Result<Self, CraftError> {
        if should_read { Ok(Some(T::read_from(reader)?)) } else { Ok(None) }
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Vec<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> Result<Self, CraftError> {
        let length = VarInt::from_reader(reader)?.raw() as usize;
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read_from(reader)?)
        }
        Ok(values)
    }
}

impl<T: ProtocolRead> PredefinedProtocolRead<usize> for Vec<T> {
    fn read_from_predefined<R: io::Read>(
        reader: &mut R,
        length: usize,
    ) -> Result<Self, CraftError> {
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read_from(reader)?)
        }
        Ok(values)
    }
}
