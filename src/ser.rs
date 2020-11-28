use std::{io::Write, rc::Rc};

use serde::{ser, Serialize};

use crate::error::{Error, Result};

mod only_string_ser;

pub struct Serializer<T: Write> {
    writer: T,
}

impl<W: Write> Serializer<W> {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        if let Err(ioe) = self.writer.write(&[byte]) {
            return Err(Error::Io(Rc::new(ioe)));
        }
        Ok(())
    }
}

/// # Errors
/// Returns [`Error::FromUtf8Error`](Error::FromUtf8Error) if
/// output buffer contains invalid UTF-8 sequence.
pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut buf = Vec::new();
    to_writer(value, &mut buf)?;
    String::from_utf8(buf).map_err(|err| Error::FromUtf8Error(err))
}

pub fn to_writer<T: Serialize, W: Write>(value: &T, writer: W) -> Result<()> {
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    Ok(())
}

impl<'s, W: Write> ser::Serializer for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    #[cfg(feature = "sort_dictionary")]
    type SerializeMap = StructMapSerializer<'s, W>;
    #[cfg(not(feature = "sort_dictionary"))]
    type SerializeMap = Self;
    #[cfg(feature = "sort_dictionary")]
    type SerializeStruct = StructMapSerializer<'s, W>;
    #[cfg(not(feature = "sort_dictionary"))]
    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        #[cfg(feature = "bool")]
        return self.serialize_u64(v as u64);
        #[cfg(not(feature = "bool"))]
        return Err(Error::Message(
            "bool is not supported in bencoding, hint: use `bool` feature".to_string(),
        ));
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.write_byte(b'i')?;
        let _ = itoa::write(&mut self.writer, v).map_err(|ioe| Error::Io(std::rc::Rc::new(ioe)))?;
        self.write_byte(b'e')?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.write_byte(b'i')?;
        let _ = itoa::write(&mut self.writer, v).map_err(|ioe| Error::Io(std::rc::Rc::new(ioe)))?;
        self.write_byte(b'e')?;
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::FloatingPointNotSupported)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::FloatingPointNotSupported)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buf = [0u8; 4];
        let str = v.encode_utf8(&mut buf);
        self.serialize_bytes(&str.as_bytes())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    /// Serializes bytes as `Byte String`
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let length = v.len();
        let map_err = |ioe| Error::Io(Rc::new(ioe));
        itoa::write(&mut self.writer, length).map_err(map_err)?;

        self.writer.write(&[b':']).map_err(map_err)?;
        // TODO: should I use `write_all()`?
        self.writer.write(v).map_err(map_err)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        #[cfg(not(feature = "none_is_err"))]
        return Ok(());
        #[cfg(feature = "none_is_err")]
        return Err(Error::Message(
            "got `None`, but feature `none_is_err` enabled",
        ));
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        // Serialize unit as nothing
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        use ser::SerializeMap;
        let mut ms = self.serialize_map(None)?;
        ms.serialize_key(name)?;
        ms.serialize_value(variant)?;
        ms.end()?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(&mut *self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.write_byte(b'l')?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_byte(b'd')?;
        #[cfg(feature = "sort_dictionary")]
        let ret = StructMapSerializer::new(self);
        #[cfg(not(feature = "sort_dictionary"))]
        let ret = self;
        Ok(ret)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.write_byte(b'd')?;
        Ok(self)
    }
}
impl<'s, W: Write> ser::SerializeSeq for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
impl<'s, W: Write> ser::SerializeTuple for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
impl<'s, W: Write> ser::SerializeTupleStruct for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
impl<'s, W: Write> ser::SerializeTupleVariant for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
#[cfg(not(feature = "sort_dictionary"))]
impl<'s, W: Write> ser::SerializeMap for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut only_string_ser::OnlyStringSerializer { ser: self })?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
#[cfg(not(feature = "sort_dictionary"))]
impl<'s, W: Write> ser::SerializeStruct for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}
impl<'s, W: Write> ser::SerializeStructVariant for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')
    }
}

#[cfg(feature = "sort_dictionary")]
mod dict_serializer {
    use super::*;
    pub struct StructMapSerializer<'s, T: Write> {
        keys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
        parent: &'s mut Serializer<T>,
    }

    impl<'s, T: Write> StructMapSerializer<'s, T> {
        pub(super) fn new(parent: &'s mut Serializer<T>) -> Self {
            StructMapSerializer {
                keys: Vec::new(),
                values: Vec::new(),
                parent,
            }
        }

        fn sort_and_write_to_parent(mut self) -> Result<()> {
            let mut map = self
                .keys
                .iter_mut()
                .zip(self.values.iter_mut())
                .collect::<Vec<_>>();
            let map_err = |ioe| Error::Io(Rc::new(ioe));
            // TODO: find out what is raw string sort? Is it right?
            map.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
            for (key, value) in map {
                self.parent.writer.write(key).map_err(map_err)?;
                self.parent.writer.write(value).map_err(map_err)?;
            }
            self.parent.write_byte(b'e')?;
            Ok(())
        }
    }
    impl<'s, W: Write> ser::SerializeMap for StructMapSerializer<'s, W> {
        type Ok = ();

        type Error = Error;

        fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
        where
            T: Serialize,
        {
            self.keys.push(Vec::new());
            let mut temp_ser = Serializer {
                writer: self.keys.last_mut().unwrap(),
            };
            key.serialize(&mut only_string_ser::OnlyStringSerializer { ser: &mut temp_ser })?;
            Ok(())
        }

        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
        {
            self.values.push(Vec::new());
            let mut temp_ser = Serializer {
                writer: self.values.last_mut().unwrap(),
            };
            value.serialize(&mut temp_ser)?;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            self.sort_and_write_to_parent()
        }
    }
    impl<'s, W: Write> ser::SerializeStruct for StructMapSerializer<'s, W> {
        type Ok = ();

        type Error = super::Error;

        fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize,
        {
            // key
            {
                self.keys.push(Vec::new());
                let mut temp_ser = Serializer {
                    writer: self.keys.last_mut().unwrap(),
                };
                key.serialize(&mut temp_ser)?;
            }
            // value
            {
                self.values.push(Vec::new());
                let mut temp_ser = Serializer {
                    writer: self.values.last_mut().unwrap(),
                };
                value.serialize(&mut temp_ser)?;
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            self.sort_and_write_to_parent()
        }
    }
}
#[cfg(feature = "sort_dictionary")]
use dict_serializer::StructMapSerializer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&1)?, "i1e");
        assert_eq!(&to_string(&0)?, "i0e");
        assert_eq!(&to_string(&-1)?, "i-1e");

        assert_eq!(&to_string(&u64::MAX)?, "i18446744073709551615e");
        assert_eq!(&to_string(&i64::MIN)?, "i-9223372036854775808e");
        Ok(())
    }

    #[test]
    fn bools() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(feature = "bool"))]
        {
            assert!(to_string(&true).is_err());
            assert!(to_string(&false).is_err());
        }
        #[cfg(feature = "bool")]
        {
            assert_eq!(&to_string(&true)?, "i1e");
            assert_eq!(&to_string(&false)?, "i0e");
        }

        Ok(())
    }

    #[test]
    fn strings() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&"abc")?, "3:abc");
        assert_eq!(&to_string(&"")?, "0:");
        let len = 1024 * 10;
        let very_long = std::iter::repeat('r').take(len).collect::<String>();
        assert_eq!(to_string(&very_long)?, format!("{}:{}", len, very_long));
        Ok(())
    }

    #[test]
    fn bytes() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let byte_buf = serde_bytes::ByteBuf::from(vec![b'a', b'b', b'c']);
        assert_eq!(&to_string(&byte_buf)?, "3:abc");
        let bytes = serde_bytes::Bytes::new(&[]);
        assert_eq!(&to_string(&bytes)?, "0:");
        let len = 1024 * 10;
        let very_long = std::iter::repeat(b'r').take(len).collect::<Vec<u8>>();
        let very_long = serde_bytes::ByteBuf::from(very_long);
        assert_eq!(
            to_string(&very_long)?,
            format!("{}:{}", len, std::str::from_utf8(very_long.as_slice())?)
        );
        Ok(())
    }

    #[test]
    fn options() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&Some(1))?, "i1e");
        assert_eq!(&to_string(&Option::<u8>::None)?, "");
        Ok(())
    }

    #[test]
    fn unit() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&())?, "");
        Ok(())
    }

    #[test]
    fn unit_struct() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        struct EmptyInside;
        assert_eq!(&to_string(&EmptyInside)?, "");
        Ok(())
    }

    #[test]
    fn newtype_struct() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        struct Kg(u32);

        assert_eq!(to_string(&Kg(300))?, format!("i300e"));
        Ok(())
    }
    #[test]
    fn newtype_variant() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        enum E {
            N(u8),
            S(&'static str),
        }

        assert_eq!(&to_string(&E::N(1))?, "i1e");
        assert_eq!(&to_string(&E::S("buf"))?, "3:buf");
        Ok(())
    }
    #[test]
    fn sequence() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(to_string(&[1, 2, 3, 4, 5])?, format!("li1ei2ei3ei4ei5ee"));
        assert_eq!(to_string(&['a', 'b', 'c'])?, format!("l1:a1:b1:ce"));
        Ok(())
    }
    #[test]
    fn tuple() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&["one", "two"])?, "l3:one3:twoe");
        assert_eq!(&to_string(&("one", "two"))?, "l3:one3:twoe");
        Ok(())
    }

    #[test]
    fn tuple_struct() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(&to_string(&Rgb(255, 64, 64))?, "li255ei64ei64ee");

        Ok(())
    }

    #[test]
    fn tuple_variant() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        enum E {
            T(u8, u8),
        };
        assert_eq!(&to_string(&E::T(1, 2))?, "li1ei2ee");

        Ok(())
    }

    #[test]
    fn map() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;

        let mut map = HashMap::with_hasher(std::hash::BuildHasherDefault::<
            hashers::null::NullHasher,
        >::default());
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(&to_string(&map)?, "d1:ai1e1:bi2e1:ci3ee");
        {
            let mut map2 = HashMap::new();
            map2.insert(1, 2);
            map2.insert(2, 3);
            map2.insert(3, 4);
            assert!(to_string(&map2).is_err());
        }

        {
            let mut map = HashMap::with_hasher(std::hash::BuildHasherDefault::<
                hashers::fx_hash::FxHasher,
            >::default());
            map.insert("a", 1);
            map.insert("c", 3);
            map.insert("b", 2);
            #[cfg(feature = "sort_dictionary")]
            assert_eq!(&to_string(&map)?, "d1:ai1e1:bi2e1:ci3ee");
            #[cfg(not(feature = "sort_dictionary"))]
            assert_eq!(&to_string(&map)?, "d1:ci3e1:bi2e1:ai1ee");
        }

        Ok(())
    }

    #[test]
    fn struct_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;
        #[derive(Debug, Serialize)]
        struct S {
            list: [u8; 3],
            string: &'static str,
            map: HashMap<
                &'static str,
                i32,
                std::hash::BuildHasherDefault<hashers::null::NullHasher>,
            >,
        };

        let mut map = HashMap::with_hasher(std::hash::BuildHasherDefault::<
            hashers::null::NullHasher,
        >::default());
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(
            &to_string(&S {
                list: [9, 8, 7],
                string: "hello",
                map
            })?,
            "d4:listli9ei8ei7ee6:string5:hello3:mapd1:ai1e1:bi2e1:ci3eee"
        );
        Ok(())
    }
}
