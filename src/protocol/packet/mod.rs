use std::io;
use std::str::FromStr;

use crate::nbt::CompoundTag;
use crate::types::{Identifier, Position, VarInt};

mod config;
mod handshaking;
mod login;
mod play;
mod status;

pub use config::*;
use eyre::{Context, bail};
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
    pub fn length(&self) -> VarInt {
        let length = self.packet_id.len() + self.data.bytes().len();
        VarInt::new(length as i32)
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

    pub fn write_all<W: ProtocolWrite>(&mut self, value: W) {
        value.write_all(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }

    pub fn write_all_prefixed<W: PrefixedProtocolWrite>(&mut self, value: W) {
        value.prefixed_write_all(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }
    pub fn read<R: ProtocolRead>(&mut self) -> crate::error::Result<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from(&mut cursor)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn read_prefixed<R: PrefixedProtocolRead>(&mut self) -> crate::error::Result<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from_prefixed(&mut cursor)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn read_predefined<T, R: PredefinedProtocolRead<T>>(
        &mut self,
        info: T,
    ) -> crate::error::Result<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from_predefined(&mut cursor, info)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn to_writer<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }
}

impl<T: Into<Vec<u8>>> From<T> for PacketData {
    fn from(bytes: T) -> Self {
        Self { bytes: bytes.into() }
    }
}

pub trait ProtocolWrite {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()>;
}

pub trait PrefixedProtocolWrite: ProtocolWrite {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()>;
}

impl ProtocolWrite for () {
    fn write_all<W: io::Write>(&self, _writer: &mut W) -> crate::error::Result<()> {
        Ok(())
    }
}

impl PrefixedProtocolWrite for () {
    fn prefixed_write_all<W: io::Write>(&self, _writer: &mut W) -> crate::error::Result<()> {
        Ok(())
    }
}

impl ProtocolWrite for bool {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        match self {
            false => writer.write(&[0x00]).wrap_err("failed to write `false`")?,
            true => writer.write(&[0x01]).wrap_err("failed to write `true`")?,
        };
        Ok(())
    }
}

macro_rules! impl_protocol_write {
    ($type:ty, $write_err:literal) => {
        impl ProtocolWrite for $type {
            fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
                writer.write_all(&self.to_be_bytes()).wrap_err($write_err)
            }
        }
    };
}

impl_protocol_write!(u8, "failed to write u8");
impl_protocol_write!(i8, "failed to write i8");
impl_protocol_write!(u16, "failed to write u16");
impl_protocol_write!(i16, "failed to write i16");
impl_protocol_write!(u32, "failed to write u32");
impl_protocol_write!(i32, "failed to write i32");
impl_protocol_write!(u64, "failed to write u64");
impl_protocol_write!(i64, "failed to write i64");

impl ProtocolWrite for String {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        VarInt::new(self.len() as i32).to_writer(writer)?;
        writer.write_all(self.as_bytes()).wrap_err("failed to write string")?;
        Ok(())
    }
}

impl ProtocolWrite for Identifier {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        self.to_string().write_all(writer).wrap_err("failed to write identifier")
    }
}

impl ProtocolWrite for VarInt {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        self.to_writer(writer).wrap_err("failed to write varint")
    }
}

impl ProtocolWrite for CompoundTag {
    fn write_all<W: io::Write>(&self, _writer: &mut W) -> crate::error::Result<()> {
        todo!();
    }
}

impl ProtocolWrite for Position {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        i64::from(*self).write_all(writer).wrap_err("failed to write position")
    }
}

impl ProtocolWrite for Uuid {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        writer.write_all(&self.to_bytes_le()).wrap_err("failed to write uuid")
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Option<T> {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        match self {
            Some(t) => t.write_all(writer),
            None => Ok(()),
        }
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Option<T> {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
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
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        for item in self {
            item.write_all(writer)?;
        }
        Ok(())
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Vec<T> {
    fn prefixed_write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        let len = VarInt::new(self.len() as i32);
        len.write_all(writer)?;
        self.write_all(writer)?;
        Ok(())
    }
}

pub trait ProtocolRead {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self>
    where
        Self: Sized;
}

pub trait PrefixedProtocolRead {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> crate::error::Result<Self>
    where
        Self: Sized;
}

pub trait PredefinedProtocolRead<T> {
    fn read_from_predefined<R: io::Read>(reader: &mut R, info: T) -> crate::error::Result<Self>
    where
        Self: Sized;
}

impl ProtocolRead for bool {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        match u8::read_from(reader).wrap_err("failed to read bool")? {
            0x01 => Ok(true),
            0x00 => Ok(false),
            byte => bail!("invalid boolean byte: {byte}"),
        }
    }
}

macro_rules! impl_protocol_read {
    ($type:ty, $read_err:literal) => {
        impl ProtocolRead for $type {
            fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
                const SIZE: usize = (<$type>::BITS / 8) as usize;
                let mut buf = [0u8; SIZE];
                reader.read_exact(&mut buf).wrap_err($read_err)?;
                Ok(<$type>::from_be_bytes(buf))
            }
        }
    };
}

impl_protocol_read!(u8, "failed to read u8");
impl_protocol_read!(i8, "failed to read i8");
impl_protocol_read!(u16, "failed to read u16");
impl_protocol_read!(i16, "failed to read i16");
impl_protocol_read!(u32, "failed to read u32");
impl_protocol_read!(i32, "failed to read i32");
impl_protocol_read!(u64, "failed to read u64");
impl_protocol_read!(i64, "failed to read i64");

impl ProtocolRead for String {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        let length = VarInt::from_reader(reader)?.raw() as usize;

        let mut string_buf = vec![0u8; length];
        reader.read_exact(&mut string_buf).wrap_err("failed to read string")?;

        Self::from_utf8(string_buf).wrap_err("failed to read string")
    }
}

impl ProtocolRead for Identifier {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        let string = String::read_from(reader).wrap_err("failed to read identifier")?;
        Self::from_str(&string)
    }
}

impl ProtocolRead for VarInt {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        Ok(Self::from_reader(reader).wrap_err("failed to read varint")?)
    }
}

impl ProtocolRead for CompoundTag {
    fn read_from<R: io::Read>(_reader: &mut R) -> crate::error::Result<Self> {
        todo!();
    }
}

impl ProtocolRead for Position {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        Ok(i64::read_from(reader).wrap_err("failed to read position")?.into())
    }
}

impl ProtocolRead for Uuid {
    fn read_from<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes).wrap_err("failed to read uuid")?;
        Ok(Self::from_bytes(bytes))
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Option<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
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
    ) -> crate::error::Result<Self> {
        if should_read { Ok(Some(T::read_from(reader)?)) } else { Ok(None) }
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Vec<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> crate::error::Result<Self> {
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
    ) -> crate::error::Result<Self> {
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read_from(reader)?)
        }
        Ok(values)
    }
}
