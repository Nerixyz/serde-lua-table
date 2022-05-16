use super::{SerError, Serializer};
use crate::format::Formatter;
use serde::{ser, ser::Impossible, Serialize};
use std::io;

pub struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> MapKeySerializer<'a, W, F> {
    pub(crate) fn new(ser: &'a mut Serializer<W, F>) -> Self {
        Self { ser }
    }
}

impl<'a, W, F> ser::Serializer for MapKeySerializer<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;
    type SerializeSeq = Impossible<(), SerError>;
    type SerializeTuple = Impossible<(), SerError>;
    type SerializeTupleStruct = Impossible<(), SerError>;
    type SerializeTupleVariant = Impossible<(), SerError>;
    type SerializeMap = Impossible<(), SerError>;
    type SerializeStruct = Impossible<(), SerError>;
    type SerializeStructVariant = Impossible<(), SerError>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u64(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::Error::KeyMustBeStringOrNumber)
    }
}
