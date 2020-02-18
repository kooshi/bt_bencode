use serde::de::{Deserialize, MapAccess, SeqAccess, Visitor};
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;
use std::fmt;

/// A Bencoded number.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Number {
    Signed(i64),
    Unsigned(u64),
}

/// Represents valid untyped data.
///
/// It is useful when it is unknown what the data may contain (e.g. when different kinds of
/// messages can be received in a network packet).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    /// A byte string.
    ///
    /// Encoded strings can contain non-UTF-8 bytes, so a byte string is used to represent
    /// "strings".
    ByteStr(ByteBuf),
    /// An integer which can be signed or unsigned.
    Int(Number),
    /// A list of values.
    List(Vec<Value>),
    /// A dictionary of values.
    Dict(BTreeMap<ByteBuf, Value>),
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<T>(deserializer: T) -> Result<Value, T::Error>
    where
        T: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("any valid Bencode value")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Value::Int(Number::Signed(value)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Value::Int(Number::Unsigned(value)))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(Value::ByteStr(ByteBuf::from(String::from(value))))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(Value::ByteStr(ByteBuf::from(value)))
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
                Ok(Value::ByteStr(ByteBuf::from(value)))
            }

            fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
                Ok(Value::ByteStr(ByteBuf::from(value)))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut list = Vec::new();
                while let Some(elem) = visitor.next_element()? {
                    list.push(elem);
                }
                Ok(Value::List(list))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut dict = BTreeMap::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    dict.insert(key, value);
                }
                Ok(Value::Dict(dict))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_deserialize_string() -> Result<()> {
        let v: Value = crate::de::from_slice("4:spam".as_bytes())?;
        assert_eq!(v, Value::ByteStr(ByteBuf::from(String::from("spam"))));
        Ok(())
    }

    #[test]
    fn test_deserialize_integer_1() -> Result<()> {
        let input = "i3e";
        let v: Value = crate::de::from_slice(input.as_bytes())?;
        assert_eq!(v, Value::Int(Number::Unsigned(3)));
        Ok(())
    }

    #[test]
    fn test_deserialize_integer_2() -> Result<()> {
        let input = "i-3e";
        let v: Value = crate::de::from_slice(input.as_bytes())?;
        assert_eq!(v, Value::Int(Number::Signed(-3)));
        Ok(())
    }

    #[test]
    fn test_deserialize_integer_3() -> Result<()> {
        let input = "i0e";
        let v: Value = crate::de::from_slice(input.as_bytes())?;
        assert_eq!(v, Value::Int(Number::Unsigned(0)));
        Ok(())
    }

    #[test]
    fn test_deserialize_list() -> Result<()> {
        let input = "l4:spam4:eggse";
        let v: Value = crate::de::from_slice(input.as_bytes())?;
        assert_eq!(
            v,
            Value::List(vec![
                Value::ByteStr(ByteBuf::from(String::from("spam"))),
                Value::ByteStr(ByteBuf::from(String::from("eggs"))),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_dict_1() -> Result<()> {
        let input = "d3:cow3:moo4:spam4:eggse";
        let v: Value = crate::de::from_slice(input.as_bytes())?;

        let mut expected = BTreeMap::new();
        expected.insert(
            ByteBuf::from(String::from("cow")),
            Value::ByteStr(ByteBuf::from(String::from("moo"))),
        );
        expected.insert(
            ByteBuf::from(String::from("spam")),
            Value::ByteStr(ByteBuf::from(String::from("eggs"))),
        );
        assert_eq!(v, Value::Dict(expected));
        Ok(())
    }

    #[test]
    fn test_deserialize_dict_2() -> Result<()> {
        let input = "d4:spaml1:a1:bee";
        let v: Value = crate::de::from_slice(input.as_bytes())?;
        let mut expected = BTreeMap::new();
        expected.insert(
            ByteBuf::from(String::from("spam")),
            Value::List(vec![
                Value::ByteStr(ByteBuf::from(String::from("a"))),
                Value::ByteStr(ByteBuf::from(String::from("b"))),
            ]),
        );
        assert_eq!(v, Value::Dict(expected));
        Ok(())
    }
}
