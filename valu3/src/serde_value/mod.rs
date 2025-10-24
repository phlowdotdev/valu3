pub mod de;
pub mod ser;

use crate::prelude::*;
use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde::Deserializer;
use std::collections::HashMap;
use std::fmt;

/// Error type used when converting between `Value` and serde types.
#[derive(Debug)]
pub struct SerdeValueError(pub String);

impl std::fmt::Display for SerdeValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SerdeValueError: {}", self.0)
    }
}

impl std::error::Error for SerdeValueError {}

impl serde::ser::Error for SerdeValueError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeValueError(msg.to_string())
    }
}

impl serde::de::Error for SerdeValueError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeValueError(msg.to_string())
    }
}

struct ValueSerializer;

struct SeqCollector {
    elems: Vec<Value>,
}

impl SerializeSeq for SeqCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Array::from(self.elems).to_value())
    }
}

struct MapCollector {
    entries: Vec<(String, Value)>,
}

impl SerializeMap for MapCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // serialize key into a Value and expect it to be a string
        let kv = key.serialize(ValueSerializer)?;
        match kv {
            Value::String(s) => {
                // temporarily push with empty value; value filled in serialize_value
                self.entries.push((s.to_string(), Value::Null));
                Ok(())
            }
            _ => Err(SerdeValueError("map key must be a string".to_string())),
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        if let Some((_k, slot)) = self.entries.last_mut() {
            *slot = v;
            Ok(())
        } else {
            Err(SerdeValueError(
                "serialize_value called before serialize_key".to_string(),
            ))
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map: HashMap<String, Value> = HashMap::with_capacity(self.entries.len());
        for (k, v) in self.entries.into_iter() {
            map.insert(k, v);
        }
        Ok(Object::from(map).to_value())
    }
}

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = SerdeValueError;
    type SerializeSeq = SeqCollector;
    type SerializeTuple = SeqCollector;
    type SerializeTupleStruct = SeqCollector;
    type SerializeTupleVariant = TupleVariantCollector;
    type SerializeMap = MapCollector;
    type SerializeStruct = MapCollector;
    type SerializeStructVariant = StructVariantCollector;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Number::from(v).to_value())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(StringB::from(v.to_string()).to_value())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(StringB::from(v).to_value())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerdeValueError("bytes not supported".to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(StringB::from(variant).to_value())
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
        let mut map = HashMap::new();
        map.insert(variant.to_string(), value.serialize(ValueSerializer)?);
        Ok(Object::from(map).to_value())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqCollector {
            elems: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqCollector {
            elems: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SeqCollector {
            elems: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantCollector {
            variant: _variant.to_string(),
            elems: Vec::with_capacity(_len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapCollector {
            entries: Vec::new(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapCollector {
            entries: Vec::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantCollector {
            variant: _variant.to_string(),
            entries: Vec::with_capacity(_len),
        })
    }
}

struct TupleVariantCollector {
    variant: String,
    elems: Vec<Value>,
}

impl serde::ser::SerializeTupleVariant for TupleVariantCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map: HashMap<String, Value> = HashMap::with_capacity(1);
        map.insert(self.variant, Array::from(self.elems).to_value());
        Ok(Object::from(map).to_value())
    }
}

struct StructVariantCollector {
    variant: String,
    entries: Vec<(String, Value)>,
}

impl serde::ser::SerializeStructVariant for StructVariantCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.entries.push((key.to_string(), v));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut inner: HashMap<String, Value> = HashMap::with_capacity(self.entries.len());
        for (k, v) in self.entries.into_iter() {
            inner.insert(k, v);
        }
        let mut map: HashMap<String, Value> = HashMap::with_capacity(1);
        map.insert(self.variant, Object::from(inner).to_value());
        Ok(Object::from(map).to_value())
    }
}

impl serde::ser::SerializeTuple for SeqCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Array::from(self.elems).to_value())
    }
}

impl serde::ser::SerializeTupleStruct for SeqCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.elems.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Array::from(self.elems).to_value())
    }
}

impl serde::ser::SerializeStruct for MapCollector {
    type Ok = Value;
    type Error = SerdeValueError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = value.serialize(ValueSerializer)?;
        self.entries.push((key.to_string(), v));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map: HashMap<String, Value> = HashMap::with_capacity(self.entries.len());
        for (k, v) in self.entries.into_iter() {
            map.insert(k, v);
        }
        Ok(Object::from(map).to_value())
    }
}

/// Serializa qualquer `T: Serialize` diretamente para `Value` sem passar por texto.
pub fn to_value<T>(value: &T) -> Result<Value, SerdeValueError>
where
    T: Serialize + ?Sized,
{
    value.serialize(ValueSerializer)
}

struct ValueDeserializer {
    input: Value,
}

struct SeqAccessImpl {
    iter: std::vec::IntoIter<Value>,
}

impl<'de> SeqAccess<'de> for SeqAccessImpl {
    type Error = SerdeValueError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(v) = self.iter.next() {
            let de = ValueDeserializer { input: v };
            let res = seed.deserialize(de)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

struct MapAccessImpl {
    iter: std::vec::IntoIter<(String, Value)>,
    current: Option<(String, Value)>,
}

impl<'de> MapAccess<'de> for MapAccessImpl {
    type Error = SerdeValueError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((k, v)) = self.iter.next() {
            self.current = Some((k.clone(), v));
            // deserialize the key from the string
            let key_value = StringB::from(k.clone()).to_value();
            let de = ValueDeserializer { input: key_value };
            let res = seed.deserialize(de)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some((_k, v)) = self.current.take() {
            let de = ValueDeserializer { input: v };
            let res = seed.deserialize(de)?;
            Ok(res)
        } else {
            Err(SerdeValueError("value requested before key".to_string()))
        }
    }
}

struct EnumAccessImpl {
    name: String,
    value: Option<Value>,
}

struct VariantAccessImpl {
    value: Option<Value>,
}

impl<'de> serde::de::EnumAccess<'de> for EnumAccessImpl {
    type Error = SerdeValueError;
    type Variant = VariantAccessImpl;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // deserialize the variant identifier from the stored name string
        let val = StringB::from(self.name.clone()).to_value();
        let de = ValueDeserializer { input: val };
        let v = seed.deserialize(de)?;
        Ok((v, VariantAccessImpl { value: self.value }))
    }
}

impl<'de> serde::de::VariantAccess<'de> for VariantAccessImpl {
    type Error = SerdeValueError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.value.is_none() {
            Ok(())
        } else {
            Err(SerdeValueError("expected unit variant".to_string()))
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(v) = self.value {
            seed.deserialize(ValueDeserializer { input: v })
        } else {
            Err(SerdeValueError("expected newtype variant".to_string()))
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(Value::Array(arr)) = self.value {
            let seq = SeqAccessImpl {
                iter: arr.into_iter(),
            };
            visitor.visit_seq(seq)
        } else {
            Err(SerdeValueError("expected tuple variant".to_string()))
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(Value::Object(obj)) = self.value {
            let mut vec: Vec<(String, Value)> = Vec::with_capacity(obj.len());
            for (k, v) in obj.iter() {
                vec.push((k.to_string(), v.clone()));
            }
            let map = MapAccessImpl {
                iter: vec.into_iter(),
                current: None,
            };
            visitor.visit_map(map)
        } else {
            Err(SerdeValueError("expected struct variant".to_string()))
        }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer {
    type Error = SerdeValueError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Value::Null => visitor.visit_unit(),
            Value::Boolean(b) => visitor.visit_bool(b),
            Value::String(s) => visitor.visit_string(s.to_string()),
            Value::Number(n) => {
                use crate::types::number::NumberType;
                match n.number_type() {
                    NumberType::I8
                    | NumberType::I16
                    | NumberType::I32
                    | NumberType::I64
                    | NumberType::I128 => {
                        // Prefer i64 when possible, fall back to i128 if out of range
                        if let Some(i) = n.to_i64() {
                            visitor.visit_i64(i)
                        } else if n.is_i128() {
                            visitor.visit_i128(n.get_i128_unsafe())
                        } else {
                            Err(SerdeValueError("signed number out of range".to_string()))
                        }
                    }
                    NumberType::U8
                    | NumberType::U16
                    | NumberType::U32
                    | NumberType::U64
                    | NumberType::U128 => {
                        // Prefer u64 when possible, fall back to u128 if necessary
                        if let Some(u) = n.to_u64() {
                            visitor.visit_u64(u)
                        } else if n.is_u128() {
                            visitor.visit_u128(n.get_u128_unsafe())
                        } else {
                            Err(SerdeValueError("unsigned number out of range".to_string()))
                        }
                    }
                    NumberType::F32 | NumberType::F64 => {
                        if let Some(f) = n.to_f64() {
                            visitor.visit_f64(f)
                        } else {
                            Err(SerdeValueError("float number out of range".to_string()))
                        }
                    }
                    NumberType::Unknown => Err(SerdeValueError("unknown number type".to_string())),
                }
            }
            Value::Array(arr) => {
                let seq = SeqAccessImpl {
                    iter: arr.into_iter(),
                };
                visitor.visit_seq(seq)
            }
            Value::Object(obj) => {
                let mut vec: Vec<(String, Value)> = Vec::with_capacity(obj.len());
                for (k, v) in obj.iter() {
                    vec.push((k.to_string(), v.clone()));
                }
                let map = MapAccessImpl {
                    iter: vec.into_iter(),
                    current: None,
                };
                visitor.visit_map(map)
            }
            Value::Undefined => visitor.visit_unit(),
            Value::DateTime(s) => visitor.visit_string(s.to_iso8601()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // For options, treat `Value::Null` as None, otherwise provide the inner value
        match self.input {
            Value::Null => visitor.visit_none(),
            other => visitor.visit_some(ValueDeserializer { input: other }),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Support enum representations:
        // - unit variants as string: "Variant"
        // - newtype/tuple/struct variants as single-key object: { "Variant": value }
        match self.input {
            Value::String(s) => {
                // unit variant
                let name = s.to_string();
                visitor.visit_enum(EnumAccessImpl { name, value: None })
            }
            Value::Object(obj) => {
                if obj.len() == 1 {
                    let (k, v) = obj.iter().next().unwrap();
                    visitor.visit_enum(EnumAccessImpl {
                        name: k.to_string(),
                        value: Some(v.clone()),
                    })
                } else {
                    Err(SerdeValueError(
                        "invalid enum representation: expected single-key object".to_string(),
                    ))
                }
            }
            _ => Err(SerdeValueError("invalid enum representation".to_string())),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

/// Desserializa um `Value` para qualquer `T: DeserializeOwned`.
pub fn from_value<T>(value: &Value) -> Result<T, SerdeValueError>
where
    T: DeserializeOwned,
{
    T::deserialize(ValueDeserializer {
        input: value.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Simple {
        a: i32,
        b: String,
        c: Vec<f64>,
    }

    #[test]
    fn test_to_from_struct() {
        let s = Simple {
            a: 42,
            b: "hello".to_string(),
            c: vec![1.5, 2.5],
        };
        let v = crate::serde_value::to_value(&s).expect("to_value failed");
        let s2: Simple = crate::serde_value::from_value(&v).expect("from_value failed");
        assert_eq!(s, s2);
    }

    #[test]
    fn test_to_from_vec_and_option() {
        let v0 = vec![1i32, 2, 3];
        let val = crate::serde_value::to_value(&v0).expect("to_value vec failed");
        let v1: Vec<i32> = crate::serde_value::from_value(&val).expect("from_value vec failed");
        assert_eq!(v0, v1);

        let opt: Option<String> = Some("x".to_string());
        let val_opt = crate::serde_value::to_value(&opt).expect("to_value option failed");
        let opt2: Option<String> =
            crate::serde_value::from_value(&val_opt).expect("from_value option failed");
        assert_eq!(opt, opt2);
    }

    #[test]
    fn test_serde_number() {
        let value = Value::from(42u64);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from(3.14);
        let serialized = serde_json::to_string(&value).unwrap();

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from(-3.14);
        let serialized = serde_json::to_string(&value).unwrap();

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from(3.14e10);
        let serialized = serde_json::to_string(&value).unwrap();

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_string() {
        let value = Value::from("hello");
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "\"hello\"");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_array() {
        let value = Value::from(vec![
            Value::from(1u64),
            Value::from(2u64),
            Value::from(3u64),
        ]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "[1,2,3]");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_object() {
        let mut object = HashMap::new();
        object.insert("a", Value::from(1u64));
        object.insert("b", Value::from(2u64));
        object.insert("c", Value::from(3u64));
        let value = Value::from(object);
        let serialized = serde_json::to_string(&value).unwrap();

        let cases = [
            r#"{"a":1,"b":2,"c":3}"#,
            r#"{"a":1,"c":3,"b":2}"#,
            r#"{"b":2,"a":1,"c":3}"#,
            r#"{"b":2,"c":3,"a":1}"#,
            r#"{"c":3,"b":2,"a":1}"#,
            r#"{"c":3,"a":1,"b":2}"#,
        ];
        assert_eq!(cases.contains(&serialized.as_str()), true);

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_bool() {
        let value = Value::from(true);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "true");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_null() {
        let value = Value::Null;
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "null");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_serde_value() {
        let value = Value::from(42u64);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "42");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from("hello");
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "\"hello\"");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from(vec![
            Value::from(1u64),
            Value::from(2u64),
            Value::from(3u64),
        ]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "[1,2,3]");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let mut object = HashMap::new();
        object.insert("a", Value::from(1u64));
        object.insert("b", Value::from(2u64));
        object.insert("c", Value::from(3u64));
        let value = Value::from(object);
        let serialized = serde_json::to_string(&value).unwrap();
        let cases = [
            r#"{"a":1,"b":2,"c":3}"#,
            r#"{"a":1,"c":3,"b":2}"#,
            r#"{"b":2,"a":1,"c":3}"#,
            r#"{"b":2,"c":3,"a":1}"#,
            r#"{"c":3,"b":2,"a":1}"#,
            r#"{"c":3,"a":1,"b":2}"#,
        ];
        assert_eq!(cases.contains(&serialized.as_str()), true);

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::from(true);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "true");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);

        let value = Value::Null;
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "null");

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, value);
    }
}
