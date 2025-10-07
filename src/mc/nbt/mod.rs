mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{Serializer, to_nbt};

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
pub struct Nbt {
    pub compound: CompoundTag,
}

impl Nbt {
    pub fn as_named_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(TAG_COMPOUND);
        write_tag_name_bytes(&self.compound.name, &mut bytes);
        write_compound_tags_bytes(&self.compound.tags, &mut bytes);
        bytes
    }

    pub fn as_network_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(TAG_COMPOUND);
        write_compound_tags_bytes(&self.compound.tags, &mut bytes);
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct CompoundTag {
    pub name: String,
    pub tags: Vec<(String, Tag)>,
}

#[derive(Debug, Clone)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(u8, Vec<Tag>),
    Compound(Vec<(String, Tag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn tag_type_id(&self) -> u8 {
        match self {
            Tag::End => TAG_END,
            Tag::Byte(_) => TAG_BYTE,
            Tag::Short(_) => TAG_SHORT,
            Tag::Int(_) => TAG_INT,
            Tag::Long(_) => TAG_LONG,
            Tag::Float(_) => TAG_FLOAT,
            Tag::Double(_) => TAG_DOUBLE,
            Tag::ByteArray(_) => TAG_BYTE_ARRAY,
            Tag::String(_) => TAG_STRING,
            Tag::List(_, _) => TAG_LIST,
            Tag::Compound(_) => TAG_COMPOUND,
            Tag::IntArray(_) => TAG_INT_ARRAY,
            Tag::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    pub fn write_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            Tag::End => {}
            Tag::Byte(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Short(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Int(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Long(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Float(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Double(value) => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::ByteArray(value) => {
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                bytes.extend(value.iter().map(|b| *b as u8));
            }
            Tag::String(value) => {
                bytes.extend_from_slice(&(value.len() as u16).to_be_bytes());
                bytes.extend_from_slice(value.as_bytes());
            }
            Tag::List(tag_type, value) => {
                bytes.push(*tag_type);
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                for tag in value {
                    tag.write_bytes(bytes);
                }
            }
            Tag::Compound(value) => write_compound_tags_bytes(value, bytes),
            Tag::IntArray(value) => {
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                for int in value {
                    bytes.extend_from_slice(&int.to_be_bytes());
                }
            }
            Tag::LongArray(value) => {
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                for long in value {
                    bytes.extend_from_slice(&long.to_be_bytes());
                }
            }
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.write_bytes(&mut bytes);
        bytes
    }
}

fn write_compound_tags_bytes(tags: &Vec<(String, Tag)>, bytes: &mut Vec<u8>) {
    for (name, tag) in tags {
        bytes.push(tag.tag_type_id());
        if tag.tag_type_id() != TAG_END {
            write_tag_name_bytes(name, bytes);
        }
        tag.write_bytes(bytes);
    }
}

fn write_tag_name_bytes(name: &str, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&(name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
}
