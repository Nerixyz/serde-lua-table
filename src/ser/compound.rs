use super::{map_key_serializer::MapKeySerializer, SerError, Serializer};
use crate::format::Formatter;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};
use std::io;

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    Empty,
    First,
    Rest,
}

pub struct Compound<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
    state: State,
}

impl<'a, W, F> Compound<'a, W, F> {
    #[inline]
    pub(crate) fn empty(ser: &'a mut Serializer<W, F>) -> Self {
        Self {
            state: State::Empty,
            ser,
        }
    }
    #[inline]
    pub(crate) fn first(ser: &'a mut Serializer<W, F>) -> Self {
        Self {
            state: State::First,
            ser,
        }
    }
    #[inline]
    fn not_empty(&self) -> bool {
        self.state != State::Empty
    }
}

impl<'a, W, F> SerializeSeq for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser
            .formatter
            .begin_array_value(&mut self.ser.writer, self.state == State::First)?;
        self.state = State::Rest;
        value.serialize(&mut *self.ser)?;
        self.ser.formatter.end_array_value(&mut self.ser.writer)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.not_empty() {
            self.ser.formatter.end_array(&mut self.ser.writer)?;
        }
        Ok(())
    }
}

impl<'a, W, F> SerializeTuple for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, W, F> SerializeTupleStruct for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, W, F> SerializeTupleVariant for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.not_empty() {
            self.ser.formatter.end_array(&mut self.ser.writer)?;
        }
        self.ser.formatter.end_object_value(&mut self.ser.writer)?;
        self.ser.formatter.end_object(&mut self.ser.writer)?;
        Ok(())
    }
}

impl<'a, W, F> SerializeMap for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser
            .formatter
            .begin_object_key(&mut self.ser.writer, self.state == State::First)?;
        self.state = State::Rest;
        key.serialize(MapKeySerializer::new(self.ser))?;
        self.ser.formatter.end_object_key(&mut self.ser.writer)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser
            .formatter
            .begin_object_value(&mut self.ser.writer)?;
        value.serialize(&mut *self.ser)?;
        self.ser.formatter.end_object_value(&mut self.ser.writer)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.not_empty() {
            self.ser.formatter.end_object(&mut self.ser.writer)?;
        }
        Ok(())
    }
}

impl<'a, W, F> SerializeStruct for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

impl<'a, W, F> SerializeStructVariant for Compound<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.not_empty() {
            self.ser.formatter.end_object(&mut self.ser.writer)?;
        }
        self.ser.formatter.end_object_value(&mut self.ser.writer)?;
        self.ser.formatter.end_object(&mut self.ser.writer)?;
        Ok(())
    }
}
