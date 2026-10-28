//! Raw JSON helpers. Only protocol-owned fields are read or changed.
use serde::{Deserialize, Deserializer};
use serde_json::{Value, value::RawValue};
use std::collections::BTreeMap;

pub type RawObject = BTreeMap<String, Box<RawValue>>;

pub fn raw(value: &Value) -> Box<RawValue> {
    serde_json::value::to_raw_value(value).expect("a JSON value is serializable")
}

pub fn object(input: &RawValue) -> Result<RawObject, ()> {
    let mut decoder = serde_json::Deserializer::from_str(input.get());
    let value = UniqueObject::deserialize(&mut decoder).map_err(|_| ())?;
    decoder.end().map_err(|_| ())?;
    Ok(value.0)
}

/// Reject duplicate object keys at every depth without normalizing scalar values.
pub fn parse(input: &[u8]) -> Result<Box<RawValue>, ()> {
    let mut decoder = serde_json::Deserializer::from_slice(input);
    UniqueValue::deserialize(&mut decoder).map_err(|_| ())?;
    decoder.end().map_err(|_| ())?;
    serde_json::from_slice(input).map_err(|_| ())
}

pub fn change(input: &RawValue, key: &str, value: Box<RawValue>) -> Result<Box<RawValue>, ()> {
    let mut fields = object(input)?;
    fields.insert(key.to_owned(), value);
    serde_json::value::to_raw_value(&fields).map_err(|_| ())
}

pub fn field(input: &RawValue, key: &str) -> Option<Box<RawValue>> {
    object(input).ok()?.remove(key)
}

pub fn text(input: &RawValue, key: &str) -> Option<String> {
    serde_json::from_str(field(input, key)?.get()).ok()
}

pub fn scalar(input: &RawValue) -> Result<Value, ()> {
    serde_json::from_str(input.get()).map_err(|_| ())
}

struct UniqueObject(RawObject);
impl<'de> Deserialize<'de> for UniqueObject {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UniqueObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an object without duplicate keys")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut values = BTreeMap::new();
                while let Some((name, value)) = map.next_entry::<String, Box<RawValue>>()? {
                    if values.insert(name, value).is_some() {
                        return Err(serde::de::Error::custom("duplicate key"));
                    }
                }
                Ok(UniqueObject(values))
            }
        }
        decoder.deserialize_map(Visitor)
    }
}

struct UniqueValue;
impl<'de> Deserialize<'de> for UniqueValue {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UniqueValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSON without duplicate keys")
            }
            fn visit_bool<E: serde::de::Error>(self, _: bool) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_i64<E: serde::de::Error>(self, _: i64) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_u64<E: serde::de::Error>(self, _: u64) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_f64<E: serde::de::Error>(self, _: f64) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_str<E: serde::de::Error>(self, _: &str) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_string<E: serde::de::Error>(self, _: String) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> { Ok(UniqueValue) }
            fn visit_seq<M: serde::de::SeqAccess<'de>>(self, mut seq: M) -> Result<Self::Value, M::Error> {
                while seq.next_element::<UniqueValue>()?.is_some() {}
                Ok(UniqueValue)
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut names = std::collections::BTreeSet::new();
                while let Some(name) = map.next_key::<String>()? {
                    if !names.insert(name) { return Err(serde::de::Error::custom("duplicate key")); }
                    map.next_value::<UniqueValue>()?;
                }
                Ok(UniqueValue)
            }
        }
        decoder.deserialize_any(Visitor)
    }
}
