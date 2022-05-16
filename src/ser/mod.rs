mod compound;
mod error;
mod map_key_serializer;

use crate::format::{format_escaped_str_contents, CompactFormatter, Formatter, PrettyFormatter};
use compound::Compound;
pub use error::*;
use serde::Serialize;
use std::io;

pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new JSON serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, CompactFormatter)
    }
}

impl<'a, W> Serializer<W, PrettyFormatter<'a>>
where
    W: io::Write,
{
    /// Creates a new JSON pretty print serializer.
    #[inline]
    pub fn pretty(writer: W) -> Self {
        Serializer::with_formatter(writer, PrettyFormatter::new())
    }
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    /// Creates a new JSON visitor whose output will be written to the writer
    /// specified.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W: io::Write, F: Formatter> serde::Serializer for &'a mut Serializer<W, F> {
    type Ok = ();
    type Error = SerError;
    type SerializeSeq = Compound<'a, W, F>;
    type SerializeTuple = Compound<'a, W, F>;
    type SerializeTupleStruct = Compound<'a, W, F>;
    type SerializeTupleVariant = Compound<'a, W, F>;
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Compound<'a, W, F>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_bool(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i8(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i16(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i32(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i64(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u8(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u16(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u32(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u64(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f32(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f64(&mut self.writer, v)
            .map_err(SerError::Io)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        format_escaped_str(&mut self.writer, &mut self.formatter, v).map_err(SerError::Io)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(SerError::Io)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(SerError::Io)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
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
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.formatter.begin_object(&mut self.writer)?;
        self.formatter.begin_object_key(&mut self.writer, true)?;
        self.serialize_str(variant)?;
        self.formatter.end_object_key(&mut self.writer)?;
        self.formatter.begin_object_value(&mut self.writer)?;
        value.serialize(&mut *self)?;
        self.formatter.end_object_value(&mut self.writer)?;
        self.formatter.end_object(&mut self.writer)?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.formatter.begin_array(&mut self.writer)?;
        if len == Some(0) {
            self.formatter.end_array(&mut self.writer)?;
            Ok(Compound::empty(self))
        } else {
            Ok(Compound::first(self))
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.formatter.begin_object(&mut self.writer)?;
        self.formatter.begin_object_key(&mut self.writer, true)?;
        self.serialize_str(variant)?;
        self.formatter.end_object_key(&mut self.writer)?;
        self.formatter.begin_object_value(&mut self.writer)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.formatter.begin_object(&mut self.writer)?;
        if len == Some(0) {
            self.formatter.end_object(&mut self.writer)?;
            Ok(Compound::empty(self))
        } else {
            Ok(Compound::first(self))
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.formatter.begin_object(&mut self.writer)?;
        self.formatter.begin_object_key(&mut self.writer, true)?;
        self.serialize_str(variant)?;
        self.formatter.end_object_key(&mut self.writer)?;
        self.formatter.begin_object_value(&mut self.writer)?;
        self.serialize_map(Some(len))
    }
}

fn format_escaped_str<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    formatter.begin_string(writer)?;
    format_escaped_str_contents(writer, formatter, value)?;
    formatter.end_string(writer)?;
    Ok(())
}
