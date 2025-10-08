use serde::ser::Impossible;
use serde::{Serialize, ser};

use crate::mc::nbt::error::{Error, Result};
use crate::mc::nbt::{CompoundTag, Nbt, TAG_END, Tag};

pub struct Serializer {
    output: Tag,

    // TODO: We should probably use a state machine.
    current_map_key: Option<String>,
}

impl Serializer {
    pub fn new() -> Self {
        Self { output: Tag::Compound(Vec::new()), current_map_key: None }
    }
}

pub fn to_nbt<T>(name: impl Into<String>, value: &T) -> Result<Nbt>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();

    value.serialize(&mut serializer)?;

    match serializer.output {
        Tag::Compound(tags) => {
            return Ok(Nbt { compound: CompoundTag { name: name.into(), tags } });
        }
        _ => return Err(Error::Message("top-level NBT must be a compound".to_string())),
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_i8(if v { 0x01 } else { 0x00 })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output = Tag::Byte(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output = Tag::Short(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output = Tag::Int(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output = Tag::Long(v);
        Ok(())
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output = Tag::Float(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output = Tag::Double(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output = Tag::String(v.to_string());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output = Tag::ByteArray(v.iter().map(|b| *b as i8).collect());
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.output = Tag::String(variant.to_string());
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output = Tag::List(TAG_END, Vec::new());
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output = Tag::Compound(Vec::new());
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.output = Tag::Compound(Vec::new());
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            Tag::List(type_id, tags) => {
                let mut serializer = Serializer::new();
                value.serialize(&mut serializer)?;
                if *type_id == TAG_END {
                    *type_id = serializer.output.tag_type_id();
                }
                tags.push(serializer.output);
            }
            _ => unreachable!("structs are always serialized as compound tags"),
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new();
        key.serialize(&mut serializer)?;
        if let Tag::String(s) = serializer.output {
            self.current_map_key = Some(s);
        } else {
            return Err(Error::Message("map keys must serialize to strings".to_string()));
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            Tag::Compound(tags) => {
                let mut serializer = Serializer::new();
                value.serialize(&mut serializer)?;
                let key = self.current_map_key.take().ok_or_else(|| {
                    Error::Message("serialize_value called before serialize_key".to_string())
                })?;
                tags.push((key.to_string(), serializer.output));
            }
            _ => unreachable!("maps are always serialized as compound tags"),
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        match &mut self.output {
            Tag::Compound(tags) => {
                tags.push(("".to_string(), Tag::End));
            }
            _ => unreachable!("maps are always serialized as compound tags"),
        }

        self.current_map_key = None;

        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            Tag::Compound(tags) => {
                let mut serializer = Serializer::new();
                value.serialize(&mut serializer)?;
                tags.push((key.to_string(), serializer.output));
            }
            _ => unreachable!("structs are always serialized as compound tags"),
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        match &mut self.output {
            Tag::Compound(tags) => {
                tags.push(("".to_string(), Tag::End));
            }
            _ => unreachable!("structs are always serialized as compound tags"),
        }

        Ok(())
    }
}
