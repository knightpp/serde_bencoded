use std::marker::PhantomData;

use crate::error::{DeError as Error, DeResult as Result};
use serde::{
    de::{self, DeserializeSeed, Visitor},
    Deserialize,
};

pub struct Auto;
impl private::Sealed for Auto {}
impl Behaviour for Auto {
    fn deserialize_byte_string<'de, Te: Behaviour, V: Visitor<'de>>(
        de: &mut Deserializer<'de, Te>,
        visitor: V,
    ) -> Result<V::Value> {
        let bytes = de.parse_byte_string()?;
        match std::str::from_utf8(bytes).ok() {
            Some(s) => visitor.visit_borrowed_str(s),
            None => visitor.visit_borrowed_bytes(bytes),
        }
    }
}

pub struct Simple;
impl private::Sealed for Simple {}
impl Behaviour for Simple {
    fn deserialize_byte_string<'de, Te: Behaviour, V: Visitor<'de>>(
        de: &mut Deserializer<'de, Te>,
        visitor: V,
    ) -> Result<V::Value> {
        de::Deserializer::deserialize_bytes(de, visitor)
    }
}
mod private {
    pub trait Sealed {}
}
pub trait Behaviour: private::Sealed {
    fn deserialize_byte_string<'de, Te: Behaviour, V: Visitor<'de>>(
        // &self,
        de: &mut Deserializer<'de, Te>,
        visitor: V,
    ) -> Result<V::Value>;
}

pub struct Deserializer<'de, T: Behaviour> {
    input: &'de [u8],
    _marker: PhantomData<T>,
}

impl<'de> Deserializer<'de, Simple> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            _marker: PhantomData,
        }
    }
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            input: input.as_bytes(),
            _marker: PhantomData,
        }
    }
}

impl<'de> Deserializer<'de, Auto> {
    pub fn from_bytes_auto(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            _marker: PhantomData,
        }
    }
    pub fn from_str_auto(input: &'de str) -> Self {
        Deserializer {
            input: input.as_bytes(),
            _marker: PhantomData,
        }
    }
}

/// Deserializes bencoded `&str` to rust's value.
/// # Examples
/** ```
# use serde_bencoded::from_str;
# fn main() -> Result<(), Box<dyn std::error::Error>>{
assert_eq!(&from_str::<&str>("4:abcd")?, &"abcd");
assert_eq!(from_str::<i64>("i-50e")?, -50);
assert_eq!(from_str::<Vec<u64>>("li1ei2ei3ee")?, vec![1, 2, 3]);
# Ok(())
# }
```*/
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    _from_slice::<T, Simple>(s.as_bytes())
}

/// The same as [`from_str`] but `deserialize_any`
/// will deserialize byte string as `str` if
/// input bytes are valid UTF-8, otherwise as `bytes`
/// ### Note
/// Thus making impossible to deserialize advanced Serde
/// constructs like tagged enums with some penalty for
/// simpler use cases.
pub fn from_str_auto<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    _from_slice::<T, Auto>(s.as_bytes())
}

fn _from_slice<'a, T, B: Behaviour>(slice: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::<B> {
        input: slice,
        _marker: PhantomData,
    };
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::SyntaxError(deserializer.input[0], None))
    }
}

/// Deserializes bencoded bytes to rust's value.
/// # Examples
/** ```
# use serde_bencoded::from_bytes;
# fn main() -> Result<(), Box<dyn std::error::Error>>{
assert_eq!(&from_bytes::<&str>(b"4:abcd")?, &"abcd");
assert_eq!(from_bytes::<i64>(b"i-50e")?, -50);
assert_eq!(from_bytes::<Vec<u64>>(b"li1ei2ei3ee")?, vec![1, 2, 3]);
# Ok(())
# }
```*/
pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    _from_slice::<T, Simple>(b)
}
/// The same as [`from_bytes`] but `deserialize_any`
/// will deserialize byte string as `str` if
/// input bytes are valid UTF-8, otherwise as `bytes`.
/// ### Note
/// Thus making impossible to deserialize advanced Serde
/// constructs like tagged enums with some penalty for
/// simpler use cases.
pub fn from_bytes_auto<'a, T>(b: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    _from_slice::<T, Auto>(b)
}

impl<'de, T: Behaviour> Deserializer<'de, T> {
    fn peek_next(&self) -> Result<u8> {
        self.input.first().copied().ok_or(Error::UnexpectedEof)
    }
    fn peek_second(&self) -> Result<u8> {
        let mut iter = self.input.iter();
        iter.next().ok_or(Error::UnexpectedEof)?;
        let val = iter.next().ok_or(Error::UnexpectedEof)?;
        Ok(*val)
    }
    fn advance(&mut self) -> Result<u8> {
        let ret = self.peek_next();
        self.input = &self.input[1..];
        ret
    }

    fn parse_byte_string(&mut self) -> Result<&'de [u8]> {
        let num_bytes = self.advance_to(b':')?;
        let num_bytes: usize = btoi::btoi(num_bytes)?;
        let bytes = self.advance_by(num_bytes)?;
        Ok(bytes)
    }

    fn advance_by(&mut self, len: usize) -> Result<&'de [u8]> {
        let ret = &self.input[0..len];
        self.input = &self.input[(ret.len())..];
        Ok(ret)
    }
    #[inline(always)]
    fn advance_to_e(&mut self) -> Result<&'de [u8]> {
        self.advance_to(b'e')
    }
    fn advance_to(&mut self, byte: u8) -> Result<&'de [u8]> {
        let ret = slice_while(self.input, byte)?;
        self.input = &self.input[(ret.len() + 1)..];
        Ok(ret)
    }
}
/// Takes everything while `!= 'e'`
/// # Return
/// Can return empty slice (`.len` == 0).
fn slice_while(bytes: &[u8], end_byte: u8) -> Result<&[u8]> {
    bytes
        .iter()
        .enumerate()
        .find(|(_, x)| **x == end_byte)
        .map(|(i, _)| &bytes[0..i])
        .ok_or(Error::UnexpectedEof)
}

impl<'de, 'a, T: Behaviour> de::Deserializer<'de> for &'a mut Deserializer<'de, T> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_next()? {
            b'i' if self.peek_second()? == b'-' => self.deserialize_i64(visitor),
            b'i' => self.deserialize_u64(visitor),
            b'l' => self.deserialize_seq(visitor),
            b'd' => self.deserialize_map(visitor),
            b'0'..=b'9' => T::deserialize_byte_string(self, visitor),
            // b'e' => {
            //     self.advance()?;
            //     // visitor.visit_unit()
            // }
            other => Err(Error::SyntaxError(other, None)),
        }
    }
    serde::forward_to_deserialize_any! {u8 u16 u32 i8 i16 i32 identifier ignored_any }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.advance()? == b'i' {
            let b = self.advance_to_e()?;
            if b.len() != 1 || ![b'0', b'1'].contains(&b[0]) {
                Err(Error::Message(
                    "expected integer between `0` to `1`".to_string(),
                ))
            } else {
                visitor.visit_bool(b[0] == b'1')
            }
        } else {
            Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let marker = self.advance()?;
        if marker != b'i' {
            return Err(Error::SyntaxError(marker, Some(b'i')));
        }
        visitor.visit_i64(btoi::btoi(self.advance_to_e()?)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let marker = self.advance()?;
        if marker != b'i' {
            return Err(Error::SyntaxError(marker, Some(b'i')));
        }
        visitor.visit_u64(btoi::btoi(self.advance_to_e()?)?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Message("`f32` is not supported".to_string()))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Message("`f64` is not supported".to_string()))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.parse_byte_string()?;
        if s.len() > 4 {
            return Err(Error::ExpectedCharString);
        }
        let mut chars = std::str::from_utf8(s)?.chars();
        if chars.clone().count() != 1 {
            return Err(Error::ExpectedCharString);
        }
        let ch = chars.next().expect("this should not happen!!!");
        visitor.visit_char(ch)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.parse_byte_string()?;
        let s = std::str::from_utf8(bytes)?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_byte_string()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.advance_by(2)?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let decoded_name = self.parse_byte_string()?;
        if name.as_bytes() != decoded_name {
            Err(Error::ExpectedUnitStructName)
        } else {
            visitor.visit_unit()
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let peek = self.advance()?;
        if peek == b'l' {
            visitor.visit_seq(ListAccess { de: self })
        } else {
            Err(Error::SyntaxError(peek, Some(b'l')))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let ret = self.deserialize_seq(visitor)?;
        self.advance()?;
        Ok(ret)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let next = self.advance()?;
        if next == b'd' {
            visitor.visit_map(self)
        } else {
            Err(Error::SyntaxError(next, Some(b'd')))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_next()? {
            b'd' => {
                self.advance()?;
                let res = visitor.visit_enum(&mut *self)?;
                if self.advance()? != b'e' {
                    Err(Error::ExpectedEndOfDictionary)
                } else {
                    Ok(res)
                }
            }
            b'0'..=b'9' => {
                use de::IntoDeserializer;
                visitor
                    .visit_enum(std::str::from_utf8(self.parse_byte_string()?)?.into_deserializer())
            }
            _ => Err(Error::ExpectedDictionary),
        }
    }
}

struct ListAccess<'m, 'de: 'm, T: Behaviour> {
    de: &'m mut Deserializer<'de, T>,
}
impl<'de, 'm, Te: Behaviour> de::SeqAccess<'de> for ListAccess<'m, 'de, Te> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_next()? == b'e' {
            // TODO: safe
            self.de.advance()?;
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }
}

impl<'de, T: Behaviour> de::MapAccess<'de> for Deserializer<'de, T> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.peek_next()? == b'e' {
            // TODO: safe
            self.advance()?;

            Ok(None)
        } else {
            Ok(Some(seed.deserialize(&mut *self)?))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(seed.deserialize(&mut *self)?)
    }
}

impl<'de, Te: Behaviour> de::VariantAccess<'de> for &mut Deserializer<'de, Te> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedString)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self, visitor)
    }
}

impl<'de, T: Behaviour> de::EnumAccess<'de> for &mut Deserializer<'de, T> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self)?;
        Ok((val, self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Ret = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn primitives() -> Ret {
        assert_eq!(from_str::<i32>("i1e")?, 1);
        assert_eq!(from_str::<i32>("i-1e")?, -1);
        assert_eq!(from_str::<u32>("i0e")?, 0);
        assert_eq!(from_str::<u32>("i1e")?, 1);
        assert_eq!(from_str::<u64>(&format!("i{}e", u64::MAX))?, u64::MAX);
        assert_eq!(from_str::<i64>(&format!("i{}e", i64::MAX))?, i64::MAX);
        assert_eq!(from_str::<i64>(&format!("i{}e", i64::MIN))?, i64::MIN);
        assert_eq!(from_str::<bool>("i1e")?, true);
        assert_eq!(from_str::<bool>("i0e")?, false);
        assert_eq!(from_str::<bool>("i2e").ok(), None);
        assert_eq!(from_str::<bool>("i-1e").ok(), None);
        assert_eq!(from_str::<char>("1:a")?, 'a');
        assert_eq!(from_str::<char>("2:ab").ok(), None);
        Ok(())
    }
    #[test]
    fn byte_string() -> Ret {
        assert_eq!(from_str::<&[u8]>("5:hello")?, b"hello");
        assert_eq!(
            from_str::<serde_bytes::ByteBuf>("5:hello")?.as_slice(),
            b"hello"
        );
        assert_eq!(from_str::<&str>("5:hello")?, "hello");
        assert_eq!(from_str::<String>("5:hello")?, "hello".to_string());

        Ok(())
    }
    #[test]
    fn seq() -> Ret {
        assert_eq!(from_str::<Vec<u8>>("li1ei2ei3ee")?, vec![1, 2, 3]);
        assert_eq!(from_str::<Vec<&str>>("l1:a1:b1:ce")?, vec!["a", "b", "c"]);
        assert_eq!(
            from_str::<(&str, &str, &str)>("l1:a1:b1:ce")?,
            ("a", "b", "c")
        );
        assert_eq!(from_str::<[&str; 3]>("l1:a1:b1:ce")?, ["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn dictionary() -> Ret {
        use std::collections::HashMap;
        let hm = from_str::<HashMap<&str, i64>>("d1:ai1e1:bi2e1:ci3ee")?;
        let mut test_hm = HashMap::new();
        test_hm.insert("a", 1);
        test_hm.insert("b", 2);
        test_hm.insert("c", 3);
        assert_eq!(hm, test_hm);
        Ok(())
    }

    #[test]
    fn options() -> Ret {
        assert_eq!(from_str::<Option<i32>>("")?, None);
        assert_eq!(from_str::<Option<i32>>("i55e")?, Some(55));
        Ok(())
    }

    #[test]
    fn unit() -> Ret {
        assert_eq!(from_str::<()>(":0")?, ());
        Ok(())
    }

    #[test]
    fn unit_struct() -> Ret {
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct EmptyInside;
        assert_eq!(from_str::<EmptyInside>("11:EmptyInside")?, EmptyInside);
        Ok(())
    }
    #[test]
    fn unit_variant() -> Ret {
        use serde::Serialize;
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
        enum E {
            A,
            B,
        };
        assert_eq!(from_str::<E>("1:A")?, E::A);
        assert_eq!(from_str::<E>("1:B")?, E::B);
        Ok(())
    }
    #[test]
    fn newtype_struct() -> Ret {
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Kg(u32);
        assert_eq!(from_str::<Kg>("i100e")?, Kg(100));
        Ok(())
    }

    #[test]
    fn newtype_variant() -> Ret {
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        enum E {
            N(u8),
            S(&'static str),
        }
        assert_eq!(from_str::<E>("d1:Ni1ee")?, E::N(1));
        assert_eq!(from_str::<E>("d1:S3:bufe")?, E::S("buf"));
        Ok(())
    }

    #[test]
    fn nested_enum_adjacently_tagged_auto() -> Ret {
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        #[serde(tag = "t", content = "c")]
        enum E {
            N(u8),
        }
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        #[serde(tag = "y")]
        enum K {
            E(E),
        }
        assert_eq!(from_str_auto::<K>("d1:y1:E1:t1:N1:ci1ee")?, K::E(E::N(1)));
        Ok(())
    }

    #[test]
    fn test_auto() -> Ret {
        #[derive(Debug, PartialEq, Eq)]
        struct TestAuto {
            string: String,
            bytes: Box<[u8]>,
        }

        struct TestAutoVisitor;
        impl<'de> Visitor<'de> for TestAutoVisitor {
            type Value = TestAuto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut string = None;
                let mut bytes: Option<&[u8]> = None;

                match map.next_key()?.unwrap() {
                    "string" => string = Some(map.next_value()?),
                    "bytes" => bytes = Some(map.next_value()?),
                    _ => panic!(),
                };
                match map.next_key()?.unwrap() {
                    "string" => string = Some(map.next_value()?),
                    "bytes" => bytes = Some(map.next_value()?),
                    _ => panic!(),
                };
                assert!(map.next_key::<()>()?.is_none());
                Ok(TestAuto {
                    bytes: bytes.unwrap().to_vec().into_boxed_slice(),
                    string: string.unwrap(),
                })
            }
        }

        impl<'de> de::Deserialize<'de> for TestAuto {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(TestAutoVisitor)
            }
        };
        let input = b"d6:string4:12345:bytes4:\x01\x02\x03\xFFe";
        let de = from_bytes_auto::<TestAuto>(input)?;
        assert_eq!(
            de,
            TestAuto {
                string: String::from("1234"),
                bytes: vec![1, 2, 3, 0xFF].into_boxed_slice(),
            }
        );
        Ok(())
    }
}
