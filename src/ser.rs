use crate::error::{SerError as Error, SerResult as Result};
use serde::{ser, Serialize};
use std::io::Write;

mod only_string_ser;

pub struct Serializer<T: Write> {
    writer: T,
    int_buf: itoa::Buffer,
}

impl<T: Write> Serializer<T> {
    fn new(writer: T) -> Self {
        Serializer {
            writer,
            int_buf: itoa::Buffer::new(),
        }
    }
}

impl<W: Write> Serializer<W> {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.writer.write_all(&[byte])?;
        Ok(())
    }
}

/// Serializes rust's type to bencode string
/// # Examples
/** ```
# use serde_bencoded::to_string;
# fn main() -> Result<(), Box<dyn std::error::Error>>{
assert_eq!(&to_string(&"abcd")?, "4:abcd");
assert_eq!(&to_string(&123)?, "i123e");
assert_eq!(&to_string(&vec![1, 2, 3])?, "li1ei2ei3ee");
# Ok(())
# }
``` */
/// # Errors
/// Returns [`Error::FromUtf8Error`](Error::FromUtf8Error) if
/// output buffer contains invalid UTF-8 sequence. Use
/// [`to_writer`](to_writer) or [`to_vec`](to_vec) if it is
/// undesirable.
pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut buf = Vec::new();
    to_writer(value, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}

/// Serializes rust's type to bencode using [`Write`](std::io::Write) trait
/// # Examples
/**
```
# use serde_bencoded::to_writer;
# fn main() -> Result<(), Box<dyn std::error::Error>>{
let mut buf = Vec::<u8>::new(); // `&mut Vec<u8>` implements `Write`
to_writer(&"abcd", &mut buf)?;
assert_eq!(&buf, b"4:abcd");
# Ok(())
# }
```*/
pub fn to_writer<T: Serialize, W: Write>(value: &T, writer: W) -> Result<()> {
    let mut serializer = Serializer {
        writer,
        int_buf: itoa::Buffer::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(())
}
/// Convenient function to get encoded value as bytes
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut serializer = Serializer {
        writer: &mut buf,
        int_buf: itoa::Buffer::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(buf)
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

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
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
        let str = self.int_buf.format(v);
        self.writer.write_all(str.as_bytes())?;
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
        let str = self.int_buf.format(v);
        self.writer.write_all(str.as_bytes())?;
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
        self.serialize_bytes(str.as_bytes())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.serialize_bytes(v.as_bytes())
    }

    /// Serializes bytes as `Byte String`
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let str = self.int_buf.format(v.len());
        self.writer.write_all(str.as_bytes())?;
        self.writer.write_all(&[b':'])?;
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::NoneNotSupported)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_all(b"0:")?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        variant.serialize(&mut *self)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        self.write_byte(b'd')?;
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        self.write_byte(b'e')?;
        Ok(())
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
    #[cfg(feature = "sort_dictionary")]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_byte(b'd')?;
        Ok(StructMapSerializer::new(self, len.unwrap_or(0)))
    }

    #[cfg(not(feature = "sort_dictionary"))]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_byte(b'd')?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.write_byte(b'd')?;
        variant.serialize(&mut *self)?;
        self.write_byte(b'd')?;
        Ok(self)
    }
}
impl<'s, W: Write> ser::SerializeSeq for &'s mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
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

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
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

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
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

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_byte(b'e')?;
        self.write_byte(b'e')?;
        Ok(())
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
        pub(super) fn new(parent: &'s mut Serializer<T>, len: usize) -> Self {
            StructMapSerializer {
                keys: Vec::with_capacity(len),
                values: Vec::with_capacity(len),
                parent,
            }
        }

        fn sort_and_write_to_parent(mut self) -> Result<()> {
            let mut map = self
                .keys
                .iter_mut()
                .zip(self.values.iter_mut())
                .collect::<Vec<_>>();
            // seems we need to skip bencode header of the string
            map.sort_by(|(k1, _), (k2, _)| {
                // if we are here than keys should already be valid bencode strings
                // so it's safe to unwrap
                let k1s = k1.iter().position(|x| *x == b':').unwrap() + 1;
                let k2s = k2.iter().position(|x| *x == b':').unwrap() + 1;
                k1[k1s..].cmp(&k2[k2s..])
            });
            for (key, value) in map {
                self.parent.writer.write_all(key)?;
                self.parent.writer.write_all(value)?;
            }
            self.parent.write_byte(b'e')?;
            Ok(())
        }
    }
    impl<'s, W: Write> ser::SerializeMap for StructMapSerializer<'s, W> {
        type Ok = ();

        type Error = Error;

        fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where
            T: Serialize + ?Sized,
        {
            let mut v = Vec::new();
            let mut temp_ser = Serializer::new(&mut v);
            key.serialize(&mut only_string_ser::OnlyStringSerializer { ser: &mut temp_ser })?;
            self.keys.push(v);
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize + ?Sized,
        {
            let mut v = Vec::new();
            let mut temp_ser = Serializer::new(&mut v);
            value.serialize(&mut temp_ser)?;
            self.values.push(v);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            self.sort_and_write_to_parent()
        }
    }
    impl<'s, W: Write> ser::SerializeStruct for StructMapSerializer<'s, W> {
        type Ok = ();

        type Error = super::Error;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize + ?Sized,
        {
            let key = {
                let mut buf = Vec::new();
                let mut temp_ser = Serializer::new(&mut buf);
                key.serialize(&mut temp_ser)?;
                buf
            };
            let value = {
                let mut buf = Vec::new();
                let mut temp_ser = Serializer::new(&mut buf);
                value.serialize(&mut temp_ser)?;
                buf
            };
            self.keys.push(key);
            self.values.push(value);
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
        assert_eq!(&to_string(&true)?, "i1e");
        assert_eq!(&to_string(&false)?, "i0e");
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
        let none = to_string(&Option::<u8>::None);
        assert!(none.is_err());
        assert_eq!(none.unwrap_err(), Error::NoneNotSupported);
        Ok(())
    }

    #[test]
    fn unit() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(&to_string(&())?, "0:");
        Ok(())
    }

    #[test]
    fn unit_struct() -> std::result::Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Serialize)]
        struct EmptyInside;
        assert_eq!(&to_string(&EmptyInside)?, "11:EmptyInside");
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

        assert_eq!(&to_string(&E::N(1))?, "d1:Ni1ee");
        assert_eq!(&to_string(&E::S("buf"))?, "d1:S3:bufe");
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
        }
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

    /// map with `Option` values are not supported. You will need to write
    /// a custom ser/de helpers for that.
    #[test]
    fn map_of_options() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;

        let mut map = HashMap::with_hasher(std::hash::BuildHasherDefault::<
            hashers::fx_hash::FxHasher,
        >::default());
        map.insert("e", None);
        map.insert("d", Some(8));
        map.insert("c", None);
        map.insert("b", None);
        map.insert("a", Some(3));
        let b = to_string(&map);
        assert!(b.is_err());
        assert_eq!(b.unwrap_err(), Error::NoneNotSupported);
        // assert_eq!(&to_string(&map)?, "d1:ai3e1:di8ee");

        Ok(())
    }

    #[test]
    fn struct_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;
        #[derive(Debug, Serialize)]
        struct S {
            unit: (),
            list: [u8; 3],
            string: &'static str,
            map: HashMap<
                &'static str,
                i32,
                std::hash::BuildHasherDefault<hashers::null::NullHasher>,
            >,
        }

        let mut map = HashMap::with_hasher(std::hash::BuildHasherDefault::<
            hashers::null::NullHasher,
        >::default());
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        assert_eq!(
            &to_string(&S {
                unit: (),
                list: [9, 8, 7],
                string: "hello",
                map
            })?,
            if cfg!(not(feature = "sort_dictionary")) {
                "d4:unit0:4:listli9ei8ei7ee6:string5:hello3:mapd1:ai1e1:bi2e1:ci3eee"
            } else {
                "d4:listli9ei8ei7ee3:mapd1:ai1e1:bi2e1:ci3ee6:string5:hello4:unit0:e"
            }
        );

        Ok(())
    }
}
