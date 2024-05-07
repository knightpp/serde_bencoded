use std::io::Write;

use serde::{ser, Serialize};

use crate::error::SerError as Error;
pub(crate) struct OnlyStringSerializer<'s, W: Write> {
    pub(crate) ser: &'s mut super::Serializer<W>,
}

impl<'s, W: Write> ser::Serializer for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}

impl<'s, W: Write> ser::SerializeMap for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
impl<'s, W: Write> ser::SerializeSeq for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}

impl<'s, W: Write> ser::SerializeStruct for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
impl<'s, W: Write> ser::SerializeStructVariant for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
impl<'s, W: Write> ser::SerializeTuple for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
impl<'s, W: Write> ser::SerializeTupleStruct for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
impl<'s, W: Write> ser::SerializeTupleVariant for &'s mut OnlyStringSerializer<'s, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::DictionaryKeyMustBeString)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::DictionaryKeyMustBeString)
    }
}
