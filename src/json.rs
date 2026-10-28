//! Bounded JSON framing checks without changing the original JSON values.
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, fmt};

pub const MAX_REQUEST_BYTES: usize = 8 * 1024 * 1024;

pub fn object(bytes: &[u8]) -> Result<Box<RawValue>, ()> {
    let raw: Box<RawValue> = serde_json::from_slice(bytes).map_err(|_| ())?;
    if !raw.get().trim_start().starts_with('{') { return Err(()); }
    inspect(&raw, 0)?;
    Ok(raw)
}

fn inspect(raw: &RawValue, depth: usize) -> Result<(), ()> {
    if depth > 128 { return Err(()); }
    match raw.get().trim_start().as_bytes().first() {
        Some(b'{') => {
            struct Members(Vec<Box<RawValue>>);
            impl<'de> Deserialize<'de> for Members {
                fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                    struct ObjectVisitor;
                    impl<'de> Visitor<'de> for ObjectVisitor {
                        type Value = Members;
                        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("an object with unique keys") }
                        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Members, M::Error> {
                            let mut names = BTreeSet::new();
                            let mut values = Vec::new();
                            while let Some(name) = map.next_key::<String>()? {
                                if !names.insert(name) { return Err(de::Error::custom("duplicate JSON key")); }
                                values.push(map.next_value()?);
                            }
                            Ok(Members(values))
                        }
                    }
                    d.deserialize_map(ObjectVisitor)
                }
            }
            let members: Members = serde_json::from_str(raw.get()).map_err(|_| ())?;
            for member in members.0 { inspect(&member, depth + 1)?; }
        }
        Some(b'[') => {
            let members: Vec<Box<RawValue>> = serde_json::from_str(raw.get()).map_err(|_| ())?;
            for member in members { inspect(&member, depth + 1)?; }
        }
        _ => {}
    }
    Ok(())
}
