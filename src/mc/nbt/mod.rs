use std::collections::HashMap;

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

pub struct Nbt {
    pub compound: CompoundTag,
}

impl Nbt {
    pub fn as_bytes(&self) -> Vec<u8> {
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

pub struct CompoundTag {
    pub name: String,
    pub tags: HashMap<String, Tag>,
}

pub enum Tag {
    End,
    Byte { value: i8 },
    Short { value: i16 },
    Int { value: i32 },
    Long { value: i64 },
    Float { value: f32 },
    Double { value: f64 },
    ByteArray { value: Vec<i8> },
    String { value: String },
    List { tag_type: u8, value: Vec<Tag> },
    Compound { value: HashMap<String, Tag> },
    IntArray { value: Vec<i32> },
    LongArray { value: Vec<i64> },
}

impl Tag {
    pub fn tag_type(&self) -> u8 {
        match self {
            Tag::End => TAG_END,
            Tag::Byte { .. } => TAG_BYTE,
            Tag::Short { .. } => TAG_SHORT,
            Tag::Int { .. } => TAG_INT,
            Tag::Long { .. } => TAG_LONG,
            Tag::Float { .. } => TAG_FLOAT,
            Tag::Double { .. } => TAG_DOUBLE,
            Tag::ByteArray { .. } => TAG_BYTE_ARRAY,
            Tag::String { .. } => TAG_STRING,
            Tag::List { .. } => TAG_LIST,
            Tag::Compound { .. } => TAG_COMPOUND,
            Tag::IntArray { .. } => TAG_INT_ARRAY,
            Tag::LongArray { .. } => TAG_LONG_ARRAY,
        }
    }

    pub fn write_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            Tag::End => {}
            Tag::Byte { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Short { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Int { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Long { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Float { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::Double { value } => bytes.extend_from_slice(&value.to_be_bytes()),
            Tag::ByteArray { value } => {
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                bytes.extend(value.iter().map(|b| *b as u8));
            }
            Tag::String { value } => {
                bytes.extend_from_slice(&(value.len() as u16).to_be_bytes());
                bytes.extend_from_slice(value.as_bytes());
            }
            Tag::List { tag_type, value } => {
                bytes.push(*tag_type);
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                for tag in value {
                    tag.write_bytes(bytes);
                }
            }
            Tag::Compound { value } => write_compound_tags_bytes(value, bytes),
            Tag::IntArray { value } => {
                bytes.extend_from_slice(&(value.len() as i32).to_be_bytes());
                for int in value {
                    bytes.extend_from_slice(&int.to_be_bytes());
                }
            }
            Tag::LongArray { value } => {
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

fn write_compound_tags_bytes(tags: &HashMap<String, Tag>, bytes: &mut Vec<u8>) {
    for (name, tag) in tags {
        bytes.push(tag.tag_type());
        write_tag_name_bytes(name, bytes);
        tag.write_bytes(bytes);
    }
    bytes.push(TAG_END);
}

fn write_tag_name_bytes(name: &str, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&(name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
}
