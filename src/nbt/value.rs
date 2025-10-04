use std::io::{self, Write};

pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;
pub const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NbtTag {
    /// Signifies the end of a TAG_Compound.
    /// It is only ever used inside a TAG_Compound,
    /// a TAG_List that has it's type id set to TAG_Compound
    /// or as the type for a TAG_List if the length is 0 or negative,
    ///  and is not named even when in a TAG_Compound
    End,
    /// A single signed byte.
    Byte { name: Option<String>, payload: i8 },
    /// A single signed, big endian 16 bit integer.
    Short { name: Option<String>, payload: i16 },
    /// A single signed, big endian 32 bit integer.
    Int { name: Option<String>, payload: i32 },
    /// A single signed, big endian 64 bit integer.
    Long { name: Option<String>, payload: i64 },
    /// A single, big endian IEEE-754 single-precision floating point number (NaN possible).
    Float { name: Option<String>, payload: f32 },
    /// A single, big endian IEEE-754 double-precision floating point number (NaN possible).
    Double { name: Option<String>, payload: f64 },
    /// A length-prefixed array of signed bytes.
    /// The prefix is a signed integer (thus 4 bytes).
    ByteArray { name: Option<String>, payload: Vec<u8> },
    /// A length-prefixed modified UTF-8 string.
    /// The prefix is an unsigned short (thus 2 bytes) signifying the length of the string in bytes.
    String { name: Option<String>, payload: String },
    /// A list of nameless tags, all of the same type.
    /// The list is prefixed with the Type ID of the items
    /// it contains (thus 1 byte), and the length of the list
    /// as a signed integer (a further 4 bytes).
    /// If the length of the list is 0 or negative,
    /// the type may be 0 (TAG_End) but otherwise it must be any other type.
    /// (The notchian implementation uses TAG_End in that situation,
    /// but another reference implementation by Mojang uses 1 instead;
    /// parsers should accept any type if the length is <= 0).
    List { name: Option<String>, type_id: u8, payload: Vec<NbtTag> },
    /// Effectively a list of named tags. Order is not guaranteed.
    Compound { name: Option<String>, payload: Vec<NbtTag> },
    /// A length-prefixed array of signed integers. The prefix is a signed integer (thus 4 bytes) and indicates the number of 4 byte integers.
    IntArray { name: Option<String>, payload: Vec<i32> },
    /// A length-prefixed array of signed longs. The prefix is a signed integer (thus 4 bytes) and indicates the number of 8 byte longs.
    LongArray { name: Option<String>, payload: Vec<i64> },
}

impl NbtTag {
    pub fn root(name: Option<String>) -> Self {
        Self::Compound { name, payload: Vec::new() }
    }

    pub fn tag_type_id(&self) -> u8 {
        match self {
            NbtTag::End => TAG_END,
            NbtTag::Byte { .. } => TAG_BYTE,
            NbtTag::Short { .. } => TAG_SHORT,
            NbtTag::Int { .. } => TAG_INT,
            NbtTag::Long { .. } => TAG_LONG,
            NbtTag::Float { .. } => TAG_FLOAT,
            NbtTag::Double { .. } => TAG_DOUBLE,
            NbtTag::ByteArray { .. } => TAG_BYTE_ARRAY,
            NbtTag::String { .. } => TAG_STRING,
            NbtTag::List { .. } => TAG_LIST,
            NbtTag::Compound { .. } => TAG_COMPOUND,
            NbtTag::IntArray { .. } => TAG_INT_ARRAY,
            NbtTag::LongArray { .. } => TAG_LONG_ARRAY,
        }
    }

    pub fn name(&self) -> Option<&String> {
        match self {
            NbtTag::End => None,
            NbtTag::Byte { name, .. } => name.as_ref(),
            NbtTag::Short { name, .. } => name.as_ref(),
            NbtTag::Int { name, .. } => name.as_ref(),
            NbtTag::Long { name, .. } => name.as_ref(),
            NbtTag::Float { name, .. } => name.as_ref(),
            NbtTag::Double { name, .. } => name.as_ref(),
            NbtTag::ByteArray { name, .. } => name.as_ref(),
            NbtTag::String { name, .. } => name.as_ref(),
            NbtTag::List { name, .. } => name.as_ref(),
            NbtTag::Compound { name, .. } => name.as_ref(),
            NbtTag::IntArray { name, .. } => name.as_ref(),
            NbtTag::LongArray { name, .. } => name.as_ref(),
        }
    }

    pub fn set_name(&mut self, new_name: Option<String>) {
        match self {
            NbtTag::End => {}
            NbtTag::Byte { name, .. } => *name = new_name,
            NbtTag::Short { name, .. } => *name = new_name,
            NbtTag::Int { name, .. } => *name = new_name,
            NbtTag::Long { name, .. } => *name = new_name,
            NbtTag::Float { name, .. } => *name = new_name,
            NbtTag::Double { name, .. } => *name = new_name,
            NbtTag::ByteArray { name, .. } => *name = new_name,
            NbtTag::String { name, .. } => *name = new_name,
            NbtTag::List { name, .. } => *name = new_name,
            NbtTag::Compound { name, .. } => *name = new_name,
            NbtTag::IntArray { name, .. } => *name = new_name,
            NbtTag::LongArray { name, .. } => *name = new_name,
        }
    }

    pub fn to_writer<W>(&self, writer: &mut W, write_mode: WriteMode) -> io::Result<()>
    where
        W: io::Write,
    {
        self.internal_to_writer(writer, write_mode, true, true)
    }

    fn internal_to_writer<W>(
        &self,
        writer: &mut W,
        write_mode: WriteMode,
        should_write_tag_type_id: bool,
        is_root: bool,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if should_write_tag_type_id {
            writer.write_all(&[self.tag_type_id()])?;
        }

        let mut write_name = |name: Option<&String>| -> io::Result<()> {
            if is_root && write_mode == WriteMode::Network {
                return Ok(());
            }

            if !is_root && name.is_none() {
                return Ok(());
            }

            let name_len = name.map(|name| name.len()).unwrap_or_default() as u16;

            writer.write_all(&name_len.to_be_bytes())?;
            writer.write_all(name.map(|name| name.as_bytes()).unwrap_or_default())?;

            Ok(())
        };

        match self {
            NbtTag::End => {}
            NbtTag::Byte { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::Short { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::Int { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::Long { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::Float { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::Double { name, payload } => {
                write_name(name.as_ref())?;
                writer.write_all(&payload.to_be_bytes())?;
            }
            NbtTag::ByteArray { name, payload } => {
                write_name(name.into())?;
                writer.write_all(&payload)?;
            }
            NbtTag::String { name, payload } => {
                let string_length = payload.len() as u16;

                write_name(name.as_ref())?;
                writer.write_all(&string_length.to_be_bytes())?;
                writer.write_all(&payload.as_bytes())?;
            }
            NbtTag::List { name, type_id, payload } => {
                let list_length = payload.len() as i32;
                let type_id = match list_length {
                    0 => TAG_END,
                    _ => *type_id,
                };

                write_name(name.as_ref())?;
                writer.write_all(&[type_id])?;
                writer.write_all(&list_length.to_be_bytes())?;

                for item in payload {
                    item.internal_to_writer(writer, WriteMode::Standard, false, false)?;
                }
            }
            NbtTag::Compound { name, payload } => {
                write_name(name.as_ref())?;

                for item in payload {
                    item.to_writer(writer, WriteMode::Standard)?;
                }

                if payload.last().is_some_and(|last| last.tag_type_id() != TAG_END) {
                    NbtTag::End.internal_to_writer(writer, WriteMode::Standard, true, false)?;
                }
            }
            NbtTag::IntArray { name, payload } => {
                let list_length = payload.len() as i32;

                write_name(name.as_ref())?;
                writer.write_all(&list_length.to_be_bytes())?;

                for item in payload {
                    writer.write_all(&item.to_be_bytes())?;
                }
            }
            NbtTag::LongArray { name, payload } => {
                let list_length = payload.len() as i32;

                write_name(name.as_ref())?;
                writer.write_all(&list_length.to_be_bytes())?;

                for item in payload {
                    writer.write_all(&item.to_be_bytes())?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WriteMode {
    Standard,
    Network,
}
