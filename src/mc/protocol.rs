use std::io;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

use crate::mc::nbt::Nbt;
use crate::mc::registry::ResourceLocation;
use crate::mc::types::{Identifier, VarInt};

pub trait BytesExt: Buf {
    fn try_get_bool(&mut self) -> io::Result<bool>;

    fn try_get_prefixed_string(&mut self) -> io::Result<String>;

    fn try_get_identifier(&mut self) -> io::Result<Identifier>;

    fn try_get_resource_location(&mut self) -> io::Result<ResourceLocation>;

    fn try_get_varint(&mut self) -> io::Result<VarInt>;

    fn try_get_named_nbt(&mut self) -> io::Result<Nbt>;

    fn try_get_network_nbt(&mut self) -> io::Result<Nbt>;

    fn try_get_uuid(&mut self) -> io::Result<Uuid>;

    fn try_get_prefixed_array<T, F>(&mut self, f: F) -> io::Result<Vec<T>>
    where
        F: Fn(&mut Self) -> io::Result<T>;

    fn try_get_prefixed_option<T, F>(&mut self, f: F) -> io::Result<Option<T>>
    where
        F: Fn(&mut Self) -> io::Result<T>;

    fn try_get_bytes_array<const N: usize>(&mut self) -> io::Result<[u8; N]>;
}

impl BytesExt for Bytes {
    fn try_get_bool(&mut self) -> io::Result<bool> {
        Ok(self.try_get_u8()? != 0x00)
    }

    fn try_get_prefixed_string(&mut self) -> io::Result<String> {
        let length = self.try_get_varint()?.raw() as usize;
        if self.len() < length {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "not enough bytes for string: length prefix of {} larger than {} remaining bytes",
                    length,
                    self.len()
                ),
            ));
        }
        let string_bytes = self.split_to(length);
        String::from_utf8(string_bytes.to_vec()).map_err(io::Error::other)
    }

    fn try_get_identifier(&mut self) -> io::Result<Identifier> {
        let string = self.try_get_prefixed_string()?;
        string.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn try_get_resource_location(&mut self) -> io::Result<ResourceLocation> {
        let string = self.try_get_prefixed_string()?;
        string.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn try_get_varint(&mut self) -> io::Result<VarInt> {
        let mut value = 0;
        let mut position = 0;

        while let Ok(current_byte) = self.try_get_u8() {
            value |= ((current_byte & VarInt::SEGMENT_BITS) as i32) << position;
            if (current_byte & VarInt::CONTINUE_BIT) == 0 {
                break;
            }
            position += 7;
            if position >= 32 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt is too big"));
            }
        }

        Ok(VarInt::new(value))
    }

    fn try_get_named_nbt(&mut self) -> io::Result<Nbt> {
        todo!()
    }

    fn try_get_network_nbt(&mut self) -> io::Result<Nbt> {
        todo!()
    }

    fn try_get_uuid(&mut self) -> io::Result<Uuid> {
        if self.remaining() < 16 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "not enough bytes for Uuid"));
        }
        let mut buf = [0u8; 16];
        self.copy_to_slice(&mut buf);
        Ok(Uuid::from_bytes(buf))
    }

    fn try_get_prefixed_array<T, F>(&mut self, f: F) -> io::Result<Vec<T>>
    where
        F: Fn(&mut Self) -> io::Result<T>,
    {
        let length = self.try_get_varint()?.raw() as usize;
        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(f(self)?);
        }
        Ok(values)
    }

    fn try_get_prefixed_option<T, F>(&mut self, f: F) -> io::Result<Option<T>>
    where
        F: Fn(&mut Self) -> io::Result<T>,
    {
        let has_value = self.try_get_bool()?;
        if has_value { Ok(Some(f(self)?)) } else { Ok(None) }
    }

    fn try_get_bytes_array<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        if self.remaining() < N {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("not enough bytes for array of length {N}"),
            ));
        }
        let mut buf = [0u8; N];
        self.copy_to_slice(&mut buf);
        Ok(buf)
    }
}

pub trait BytesMutExt {
    fn put_bool(&mut self, value: bool);

    fn put_prefixed_string(&mut self, string: &str);

    fn put_identifier(&mut self, identifier: &Identifier);

    fn put_resource_location(&mut self, rec_loc: &ResourceLocation);

    fn put_varint(&mut self, varint: VarInt);

    fn put_named_nbt(&mut self, nbt: &Nbt);

    fn put_network_nbt(&mut self, nbt: &Nbt);

    fn put_uuid(&mut self, uuid: &Uuid);

    fn put_prefixed_array<T, F>(&mut self, array: &[T], f: F)
    where
        F: Fn(&T, &mut Self);

    fn put_prefixed_option<T, F>(&mut self, option: &Option<T>, f: F)
    where
        F: Fn(&T, &mut Self);
}

impl BytesMutExt for BytesMut {
    fn put_bool(&mut self, value: bool) {
        self.put_u8(if value { 0x01 } else { 0x00 });
    }

    fn put_prefixed_string(&mut self, string: &str) {
        let length = VarInt::new(string.len() as i32);
        self.put_varint(length);
        self.put_slice(string.as_bytes());
    }

    fn put_identifier(&mut self, identifier: &Identifier) {
        self.put_prefixed_string(&identifier.to_string());
    }

    fn put_resource_location(&mut self, rec_loc: &ResourceLocation) {
        self.put_prefixed_string(&rec_loc.to_string());
    }

    fn put_varint(&mut self, varint: VarInt) {
        self.extend_from_slice(&varint.to_bytes());
    }

    fn put_named_nbt(&mut self, nbt: &Nbt) {
        self.put(nbt.as_named_bytes().as_slice());
    }

    fn put_network_nbt(&mut self, nbt: &Nbt) {
        self.put(nbt.as_network_bytes().as_slice());
    }

    fn put_uuid(&mut self, uuid: &Uuid) {
        self.extend_from_slice(uuid.as_bytes());
    }

    fn put_prefixed_array<T, F>(&mut self, array: &[T], f: F)
    where
        F: Fn(&T, &mut Self),
    {
        let length = VarInt::new(array.len() as i32);
        self.put_varint(length);
        for item in array {
            f(item, self);
        }
    }

    fn put_prefixed_option<T, F>(&mut self, option: &Option<T>, f: F)
    where
        F: Fn(&T, &mut Self),
    {
        let has_value = option.is_some();
        self.put_bool(has_value);
        if let Some(value) = option {
            f(value, self);
        }
    }
}
