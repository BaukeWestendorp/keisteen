use std::io;
use std::str::FromStr;

use eyre::{Context, bail};
use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::nbt;
use crate::types::{Identifier, Position, VarInt};

pub use config::*;
pub use handshaking::*;
pub use login::*;
pub use play::*;
pub use status::*;

mod config;
mod handshaking;
mod login;
mod play;
mod status;

pub trait ServerboundPacket {
    fn decode(raw: RawPacket) -> KeisteenResult<Self>
    where
        Self: Sized;

    fn handle_invalid_packet_id(id: i32) -> KeisteenResult<Self>
    where
        Self: Sized,
    {
        bail!("invalid packet id: {id:#04x}");
    }
}

pub trait ClientboundPacket {
    fn packet_id(&self) -> i32;

    fn encode(self, data: &mut PacketData);
}

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

    pub fn write<W: ProtocolWrite>(&mut self, value: W) {
        value.write(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }

    pub fn write_prefixed<W: PrefixedProtocolWrite>(&mut self, value: W) {
        value.write_prefixed(&mut self.bytes).expect("writing into Vec<u8> should not error")
    }

    pub fn read<R: ProtocolRead>(&mut self) -> KeisteenResult<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from(&mut cursor)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn read_prefixed<R: PrefixedProtocolRead>(&mut self) -> KeisteenResult<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from_prefixed(&mut cursor)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn read_predefined<T, R: PredefinedProtocolRead<T>>(
        &mut self,
        info: T,
    ) -> KeisteenResult<R> {
        let mut cursor = io::Cursor::new(&self.bytes);
        let result = R::read_from_predefined(&mut cursor, info)?;
        self.bytes.drain(..cursor.position() as usize);
        Ok(result)
    }

    pub fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(&self.bytes)?;
        Ok(())
    }
}

impl<T: Into<Vec<u8>>> From<T> for PacketData {
    fn from(bytes: T) -> Self {
        Self { bytes: bytes.into() }
    }
}

pub trait ProtocolWrite {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()>;
}

pub trait PrefixedProtocolWrite {
    fn write_prefixed<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()>;
}

impl ProtocolWrite for () {
    fn write<W: io::Write>(&self, _writer: &mut W) -> KeisteenResult<()> {
        Ok(())
    }
}

impl PrefixedProtocolWrite for () {
    fn write_prefixed<W: io::Write>(&self, _writer: &mut W) -> KeisteenResult<()> {
        Ok(())
    }
}

impl ProtocolWrite for bool {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
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
            fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
                writer.write(&self.to_be_bytes()).wrap_err($write_err)?;
                Ok(())
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
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        VarInt::new(self.len() as i32).to_writer(writer)?;
        writer.write(self.as_bytes()).wrap_err("failed to write string")?;
        Ok(())
    }
}

impl ProtocolWrite for &str {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        VarInt::new(self.len() as i32).to_writer(writer)?;
        writer.write(self.as_bytes()).wrap_err("failed to write string")?;
        Ok(())
    }
}

impl ProtocolWrite for str {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        VarInt::new(self.len() as i32).to_writer(writer)?;
        writer.write(self.as_bytes()).wrap_err("failed to write string")?;
        Ok(())
    }
}

impl ProtocolWrite for Identifier {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.to_string().write(writer).wrap_err("failed to write identifier")
    }
}

impl ProtocolWrite for VarInt {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.to_writer(writer).wrap_err("failed to write varint")
    }
}

impl ProtocolWrite for nbt::NbtTag {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.to_writer(writer, nbt::WriteMode::Network).wrap_err("failed to write nbt value")
    }
}

impl ProtocolWrite for Position {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        i64::from(*self).write(writer).wrap_err("failed to write position")
    }
}

impl ProtocolWrite for Uuid {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        writer.write(&self.to_bytes_le()).wrap_err("failed to write uuid")?;
        Ok(())
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Option<T> {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        match self {
            Some(t) => t.write(writer),
            None => Ok(()),
        }
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Option<T> {
    fn write_prefixed<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        match self {
            Some(t) => {
                true.write(writer)?;
                t.write(writer)?;
            }
            None => {
                false.write(writer)?;
            }
        }
        Ok(())
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Vec<T> {
    fn write<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        for item in self {
            item.write(writer)?;
        }
        Ok(())
    }
}

impl<T: ProtocolWrite> PrefixedProtocolWrite for Vec<T> {
    fn write_prefixed<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        let len = VarInt::new(self.len() as i32);
        len.write(writer)?;
        self.write(writer)?;
        Ok(())
    }
}

pub trait ProtocolRead {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self>
    where
        Self: Sized;
}

pub trait PrefixedProtocolRead {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> KeisteenResult<Self>
    where
        Self: Sized;
}

pub trait PredefinedProtocolRead<T> {
    fn read_from_predefined<R: io::Read>(reader: &mut R, info: T) -> KeisteenResult<Self>
    where
        Self: Sized;
}

impl ProtocolRead for bool {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
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
            fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
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
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        let length = VarInt::from_reader(reader)?.raw() as usize;

        let mut string_buf = vec![0u8; length];
        reader.read_exact(&mut string_buf).wrap_err("failed to read string")?;

        Self::from_utf8(string_buf).wrap_err("failed to read string")
    }
}

impl ProtocolRead for Identifier {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        let string = String::read_from(reader).wrap_err("failed to read identifier")?;
        Self::from_str(&string)
    }
}

impl ProtocolRead for VarInt {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        Ok(Self::from_reader(reader).wrap_err("failed to read varint")?)
    }
}

impl ProtocolRead for nbt::NbtTag {
    fn read_from<R: io::Read>(_reader: &mut R) -> KeisteenResult<Self> {
        todo!();
    }
}

impl ProtocolRead for Position {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        Ok(i64::read_from(reader).wrap_err("failed to read position")?.into())
    }
}

impl ProtocolRead for Uuid {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes).wrap_err("failed to read uuid")?;
        Ok(Self::from_bytes(bytes))
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Option<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
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
    ) -> KeisteenResult<Self> {
        if should_read { Ok(Some(T::read_from(reader)?)) } else { Ok(None) }
    }
}

impl<T: ProtocolRead> PrefixedProtocolRead for Vec<T> {
    fn read_from_prefixed<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        let length = VarInt::from_reader(reader)?.raw() as usize;
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read_from(reader)?)
        }
        Ok(values)
    }
}

impl<T: ProtocolRead> PredefinedProtocolRead<usize> for Vec<T> {
    fn read_from_predefined<R: io::Read>(reader: &mut R, length: usize) -> KeisteenResult<Self> {
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read_from(reader)?)
        }
        Ok(values)
    }
}
