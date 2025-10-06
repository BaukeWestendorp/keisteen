use std::fmt::Write;
use std::io;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

use crate::mc::types::VarInt;

pub trait ProtocolRead {
    fn read(bytes: &mut Bytes) -> io::Result<Self>
    where
        Self: Sized;
}

pub trait ProtocolWrite {
    fn write(&self, bytes: &mut BytesMut);
}

pub trait ProtocolPrefixedRead {
    fn read_prefixed(bytes: &mut Bytes) -> io::Result<Self>
    where
        Self: Sized;
}

pub trait ProtocolPrefixedWrite {
    fn write_prefixed(&self, bytes: &mut BytesMut);
}

macro_rules! impl_proto_rw_primitive {
    [$(($type:ty, $try_get:ident, $put:ident)),*] => {
        $(
            impl ProtocolRead for $type {
                fn read(bytes: &mut Bytes) -> io::Result<Self> {
                    bytes.$try_get().map_err(|err| io::Error::other(err))
                }
            }

            impl ProtocolWrite for $type {
                fn write(&self, bytes: &mut BytesMut) {
                    bytes.$put(*self);
                }
            }
        )*
    };
}

impl_proto_rw_primitive![
    (u8, try_get_u8, put_u8),
    (i8, try_get_i8, put_i8),
    (u16, try_get_u16, put_u16),
    (i16, try_get_i16, put_i16),
    (u32, try_get_u32, put_u32),
    (i32, try_get_i32, put_i32),
    (u64, try_get_u64, put_u64),
    (i64, try_get_i64, put_i64),
    (f32, try_get_f32, put_f32),
    (f64, try_get_f64, put_f64)
];

impl ProtocolRead for bool {
    fn read(bytes: &mut Bytes) -> io::Result<Self> {
        Ok(bytes.try_get_u8()? != 0x00)
    }
}

impl ProtocolWrite for bool {
    fn write(&self, bytes: &mut BytesMut) {
        bytes.put_u8((*self).into());
    }
}

impl ProtocolRead for String {
    fn read(bytes: &mut Bytes) -> io::Result<Self> {
        let length = VarInt::read(bytes)?.raw() as usize;
        if bytes.len() < length {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "not enough bytes for string: length prefix of {} larger than {} remaining bytes",
                    length,
                    bytes.len()
                ),
            ));
        }
        let string_bytes = bytes.split_to(length);
        String::from_utf8(string_bytes.to_vec()).map_err(|err| io::Error::other(err))
    }
}

impl ProtocolWrite for String {
    fn write(&self, bytes: &mut BytesMut) {
        bytes.put(self.as_bytes());
        let length = VarInt::new(self.len() as i32);
        length.write(bytes);
        bytes.write_str(&self).unwrap();
    }
}

impl ProtocolWrite for &str {
    fn write(&self, bytes: &mut BytesMut) {
        let length = VarInt::new(self.len() as i32);
        length.write(bytes);
        bytes.write_str(self).unwrap();
    }
}

impl ProtocolWrite for str {
    fn write(&self, bytes: &mut BytesMut) {
        let length = VarInt::new(self.len() as i32);
        length.write(bytes);
        bytes.write_str(self).unwrap();
    }
}

impl ProtocolRead for VarInt {
    // TODO: This is duplicated in `VarInt`
    fn read(bytes: &mut Bytes) -> io::Result<Self> {
        let mut value = 0;
        let mut position = 0;

        while let Ok(current_byte) = bytes.try_get_u8() {
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
}

impl ProtocolWrite for VarInt {
    fn write(&self, bytes: &mut BytesMut) {
        bytes.extend_from_slice(&self.to_bytes());
    }
}

impl ProtocolRead for Uuid {
    fn read(bytes: &mut Bytes) -> io::Result<Self> {
        if bytes.remaining() < 16 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "not enough bytes for Uuid"));
        }
        let mut buf = [0u8; 16];
        bytes.copy_to_slice(&mut buf);
        Ok(Uuid::from_bytes(buf))
    }
}

impl ProtocolWrite for Uuid {
    fn write(&self, bytes: &mut BytesMut) {
        bytes.extend_from_slice(self.as_bytes());
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Option<T> {
    fn write(&self, bytes: &mut BytesMut) {
        match self {
            Some(value) => value.write(bytes),
            None => {}
        }
    }
}

impl<T: ProtocolRead> ProtocolPrefixedRead for Option<T> {
    fn read_prefixed(bytes: &mut Bytes) -> io::Result<Self> {
        let has_value = bytes.try_get_u8()? != 0x00;
        if has_value { Ok(Some(T::read(bytes)?)) } else { Ok(None) }
    }
}

impl<T: ProtocolWrite> ProtocolPrefixedWrite for Option<T> {
    fn write_prefixed(&self, bytes: &mut BytesMut) {
        let has_value = self.is_some();
        bytes.put_u8(if has_value { 1 } else { 0 });
        if let Some(value) = self {
            value.write(bytes);
        }
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Vec<T> {
    fn write(&self, bytes: &mut BytesMut) {
        for value in self {
            value.write(bytes);
        }
    }
}

impl<T: ProtocolRead> ProtocolPrefixedRead for Vec<T> {
    fn read_prefixed(bytes: &mut Bytes) -> io::Result<Self> {
        let length = VarInt::read(bytes)?.raw() as usize;
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(T::read(bytes)?);
        }
        Ok(values)
    }
}

impl<T: ProtocolWrite> ProtocolPrefixedWrite for Vec<T> {
    fn write_prefixed(&self, bytes: &mut BytesMut) {
        let length = VarInt::new(self.len() as i32);
        length.write(bytes);
        for value in self {
            value.write(bytes);
        }
    }
}
