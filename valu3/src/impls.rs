use core::panic;

use crate::prelude::*;

impl Value {
    pub fn get<T>(&self, key: T) -> Option<&Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(object) => object.get(key),
            Value::Array(array) => array.get(key.as_usize()),
            _ => panic!("Unable to get a reference to a type other than an array or object"),
        }
    }

    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(object) => object.get_mut(key),
            Value::Array(array) => array.get_mut(key.as_usize()),
            _ => {
                panic!("Unable to get a mutable reference to a type other than an array or object")
            }
        }
    }

    pub fn clean(&mut self) {
        match self {
            Value::Array(array) => array.clean(),
            Value::Object(object) => {
                object.clean();
            }
            Value::Number(number) => {
                number.clean();
            }
            _ => panic!("Unable to clean a type other than an array, object or number"),
        };
    }

    pub fn len(&self) -> usize {
        match self {
            Value::Array(array) => array.len(),
            Value::Object(object) => object.len(),
            Value::String(string) => string.len(),
            _ => panic!("Unable to get the length of a type other than an array, object or string"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Array(array) => array.is_empty(),
            Value::Object(object) => object.is_empty(),
            Value::String(string) => string.is_empty(),
            _ => panic!("Unable to check if a type other than an array, object or string is empty"),
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_undefined(&self) -> bool {
        match self {
            Value::Undefined => true,
            _ => false,
        }
    }

    pub fn as_string_b(&self) -> Option<&StringB> {
        match self {
            Value::String(string) => Some(&string),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(number) => Some(number),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_undefined(&self) -> Option<()> {
        match self {
            Value::Undefined => Some(()),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut StringB> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_number_mut(&mut self) -> Option<&mut Number> {
        match self {
            Value::Number(number) => Some(number),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Value::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Value::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_null_mut(&mut self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_undefined_mut(&mut self) -> Option<()> {
        match self {
            Value::Undefined => Some(()),
            _ => None,
        }
    }

    pub fn push<T: ToValueBehavior>(&mut self, value: T) {
        match self {
            Value::Array(array) => array.push(value.to_value()),
            _ => panic!("Unable to push values ​​into a type other than an array"),
        }
    }

    pub fn insert<T, V>(&mut self, key: T, value: V) -> Option<Value>
    where
        T: ValueKeyBehavior,
        V: ToValueBehavior,
    {
        match self {
            Value::Object(o) => o.insert(key, value.to_value()),
            _ => panic!("Unable to insert values ​​into a type other than an object"),
        }
    }
}

impl NumberBehavior for Value {
    fn set_u8(&mut self, value: u8) {
        match self {
            Value::Number(n) => n.set_u8(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_u16(&mut self, value: u16) {
        match self {
            Value::Number(n) => n.set_u16(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_u32(&mut self, value: u32) {
        match self {
            Value::Number(n) => n.set_u32(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_u64(&mut self, value: u64) {
        match self {
            Value::Number(n) => n.set_u64(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_u128(&mut self, value: u128) {
        match self {
            Value::Number(n) => n.set_u128(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_i8(&mut self, value: i8) {
        match self {
            Value::Number(n) => n.set_i8(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_i16(&mut self, value: i16) {
        match self {
            Value::Number(n) => n.set_i16(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_i32(&mut self, value: i32) {
        match self {
            Value::Number(n) => n.set_i32(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_i64(&mut self, value: i64) {
        match self {
            Value::Number(n) => n.set_i64(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_i128(&mut self, value: i128) {
        match self {
            Value::Number(n) => n.set_i128(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_f32(&mut self, value: f32) {
        match self {
            Value::Number(n) => n.set_f32(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn set_f64(&mut self, value: f64) {
        match self {
            Value::Number(n) => n.set_f64(value),
            _ => panic!("Unable to set a value other than a number"),
        }
    }

    fn get_u8(&self) -> Option<u8> {
        match self {
            Value::Number(n) => n.get_u8(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u16(&self) -> Option<u16> {
        match self {
            Value::Number(n) => n.get_u16(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u32(&self) -> Option<u32> {
        match self {
            Value::Number(n) => n.get_u32(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.get_u64(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u128(&self) -> Option<u128> {
        match self {
            Value::Number(n) => n.get_u128(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i8(&self) -> Option<i8> {
        match self {
            Value::Number(n) => n.get_i8(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i16(&self) -> Option<i16> {
        match self {
            Value::Number(n) => n.get_i16(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i32(&self) -> Option<i32> {
        match self {
            Value::Number(n) => n.get_i32(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.get_i64(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i128(&self) -> Option<i128> {
        match self {
            Value::Number(n) => n.get_i128(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_f32(&self) -> Option<f32> {
        match self {
            Value::Number(n) => n.get_f32(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.get_f64(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u8_unsafe(&self) -> u8 {
        match self {
            Value::Number(n) => n.get_u8_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u16_unsafe(&self) -> u16 {
        match self {
            Value::Number(n) => n.get_u16_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u32_unsafe(&self) -> u32 {
        match self {
            Value::Number(n) => n.get_u32_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u64_unsafe(&self) -> u64 {
        match self {
            Value::Number(n) => n.get_u64_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_u128_unsafe(&self) -> u128 {
        match self {
            Value::Number(n) => n.get_u128_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i8_unsafe(&self) -> i8 {
        match self {
            Value::Number(n) => n.get_i8_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i16_unsafe(&self) -> i16 {
        match self {
            Value::Number(n) => n.get_i16_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i32_unsafe(&self) -> i32 {
        match self {
            Value::Number(n) => n.get_i32_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i64_unsafe(&self) -> i64 {
        match self {
            Value::Number(n) => n.get_i64_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_i128_unsafe(&self) -> i128 {
        match self {
            Value::Number(n) => n.get_i128_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_f32_unsafe(&self) -> f32 {
        match self {
            Value::Number(n) => n.get_f32_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn get_f64_unsafe(&self) -> f64 {
        match self {
            Value::Number(n) => n.get_f64_unsafe(),
            _ => panic!("Unable to get a value other than a number"),
        }
    }

    fn is_i8(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i8(),
            _ => false,
        }
    }

    fn is_i16(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i16(),
            _ => false,
        }
    }

    fn is_i32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i32(),
            _ => false,
        }
    }

    fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    fn is_i128(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i128(),
            _ => false,
        }
    }

    fn is_u8(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u8(),
            _ => false,
        }
    }

    fn is_u16(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u16(),
            _ => false,
        }
    }

    fn is_u32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u32(),
            _ => false,
        }
    }

    fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    fn is_u128(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u128(),
            _ => false,
        }
    }

    fn is_f32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f32(),
            _ => false,
        }
    }

    fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            Value::Number(n) => n.is_integer(),
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self {
            Value::Number(n) => n.is_float(),
            _ => false,
        }
    }

    fn is_signed(&self) -> bool {
        match self {
            Value::Number(n) => n.is_signed(),
            _ => false,
        }
    }

    fn is_unsigned(&self) -> bool {
        match self {
            Value::Number(n) => n.is_unsigned(),
            _ => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Value::Number(n) => n.is_zero(),
            _ => false,
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Value::Number(n) => n.is_positive(),
            _ => false,
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Value::Number(n) => n.is_negative(),
            _ => false,
        }
    }

    fn number_type(&self) -> NumberType {
        match self {
            Value::Number(n) => n.number_type(),
            _ => NumberType::Unknown,
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.to_f64(),
            _ => None,
        }
    }

    fn to_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.to_i64(),
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.to_u64(),
            _ => None,
        }
    }
}

impl ObjectBehavior for Value {
    fn remove<T>(&mut self, key: &T) -> Option<Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(o) => o.remove(key),
            _ => panic!("Unable to remove a value other than an object"),
        }
    }

    fn contains_key<T>(&self, key: &T) -> bool
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(o) => o.contains_key(key),
            _ => panic!("Unable to remove a value other than an object"),
        }
    }

    fn keys(&self) -> Vec<&ValueKey> {
        match self {
            Value::Object(o) => o.keys(),
            _ => panic!("Unable to remove a value other than an object"),
        }
    }

    fn values(&self) -> Vec<&Value> {
        match self {
            Value::Object(o) => o.values(),
            _ => panic!("Unable to remove a value other than an object"),
        }
    }
}

impl ArrayBehavior for Value {
    fn pop(&mut self) -> Option<Value> {
        match self {
            Value::Array(array) => array.pop(),
            _ => panic!("Unable to pop a value other than an array"),
        }
    }
}

impl DateTimeBehavior for Value {
    fn as_date(&self) -> Option<&chrono::NaiveDate> {
        match self {
            Value::DateTime(datetime) => datetime.as_date(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn as_time(&self) -> Option<&chrono::NaiveTime> {
        match self {
            Value::DateTime(datetime) => datetime.as_time(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn as_date_time(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        match self {
            Value::DateTime(datetime) => datetime.as_date_time(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn year(&self) -> Option<i32> {
        match self {
            Value::DateTime(datetime) => datetime.year(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn month(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.month(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn day(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.day(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn hour(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.hour(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn minute(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.minute(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn second(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.second(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn timestamp(&self) -> Option<i64> {
        match self {
            Value::DateTime(datetime) => datetime.timestamp(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn timezone(&self) -> Option<chrono::Utc> {
        match self {
            Value::DateTime(datetime) => datetime.timezone(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn to_iso8601(&self) -> String {
        match self {
            Value::DateTime(datetime) => datetime.to_iso8601(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn to_rfc3339(&self) -> String {
        match self {
            Value::DateTime(datetime) => datetime.to_rfc3339(),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn add_duration(&self, duration: chrono::Duration) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            Value::DateTime(datetime) => match datetime.add_duration(duration) {
                Some(datetime) => Some(datetime.to_value()),
                None => None,
            },
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn subtract_duration(&self, duration: chrono::Duration) -> Option<Self>
    where
        Self: Sized,
    {
        match self {
            Value::DateTime(datetime) => match datetime.subtract_duration(duration) {
                Some(datetime) => Some(datetime.to_value()),
                None => None,
            },
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn duration_between(&self, other: &Self) -> Option<chrono::Duration> {
        match self {
            Value::DateTime(datetime) => datetime.duration_between(&DateTime::from(other.clone())),
            _ => panic!("Unable to get a date from a value other than a datetime"),
        }
    }

    fn from_ymd_opt(year: i32, month: u32, day: u32) -> Self {
        DateTime::from_ymd_opt(year, month, day).to_value()
    }

    fn with_ymd_and_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self {
        DateTime::with_ymd_and_hms(year, month, day, hour, min, sec).to_value()
    }

    fn now() -> Self {
        DateTime::now().to_value()
    }
}

impl StringBehavior for Value {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Value::String(string) => string.as_bytes(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Value::String(string) => string.as_str(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn as_string(&self) -> String {
        match self {
            Value::String(string) => string.as_string(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    #[cfg(feature = "cstring")]
    fn extract(&self) -> CString {
        match self {
            Value::String(string) => string.extract(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    #[cfg(not(feature = "cstring"))]
    fn extract(&self) -> String {
        match self {
            Value::String(string) => string.extract(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn to_uppercase(&self) -> Self {
        match self {
            Value::String(string) => string.to_uppercase().to_value(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn to_lowercase(&self) -> Self {
        match self {
            Value::String(string) => string.to_lowercase().to_value(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn trim(&self) -> Self {
        match self {
            Value::String(string) => string.trim().to_value(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn replace(&self, from: &str, to: &str) -> Self {
        match self {
            Value::String(string) => string.replace(from, to).to_value(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn concat<T: AsRef<str>>(&self, other: T) -> Self {
        match self {
            Value::String(string) => string.concat(other).to_value(),
            _ => panic!("Unable to get a string from a value other than a string"),
        }
    }

    fn from_utf8(value: Vec<u8>) -> Self {
        StringB::from_utf8(value).to_value()
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<T> From<T> for Value
where
    T: ToValueBehavior + PrimitiveType,
{
    fn from(value: T) -> Self {
        value.to_value()
    }
}

impl<K, V> From<Vec<(K, V)>> for Value
where
    K: ValueKeyBehavior,
    V: ToValueBehavior + PrimitiveType,
{
    fn from(value: Vec<(K, V)>) -> Self {
        let mut object = Object::default();
        for (key, value) in value {
            object.insert(key, value.to_value());
        }
        Value::Object(object)
    }
}

impl<K> From<Vec<(K, Value)>> for Value
where
    K: ValueKeyBehavior,
{
    fn from(value: Vec<(K, Value)>) -> Self {
        let mut object = Object::default();
        for (key, value) in value {
            object.insert(key, value);
        }
        Value::Object(object)
    }
}

//TODO: implement [(K, V)] and [(K, Value)]

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_value_number_behavior() {
        let value = Value::from(3.14);
        assert_eq!(value.get_f64_unsafe(), 3.14);

        let value2 = Value::from(42u32);
        assert_eq!(value2.to_u64(), Some(42));
    }

    #[test]
    fn test_value_object_behavior() {
        let mut value = Value::from(HashMap::from_iter(vec![("1", 3.14.to_value())].into_iter()));
        value.insert("2", 4.13);

        if let Some(item) = value.get_mut("1") {
            *item = 1.43.to_value();
        }

        assert_eq!(value.get("1").unwrap(), &1.43.to_value());
    }

    #[test]
    fn test_value_array_behavior() {
        let mut value = Value::from(vec![1, 2, 3]);
        value.push(4);

        if let Some(item) = value.get_mut("1") {
            *item = 1.43.to_value();
        }

        assert_eq!(value.get("1").unwrap(), &1.43.to_value());
    }

    #[test]
    fn test_value_datetime_behavior() {
        let dt_date = Value::from_ymd_opt(2023, 4, 5);
        let dt_datetime = Value::with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        assert_eq!(
            dt_date.add_duration(Duration::days(1)),
            Some(DateTime::from(NaiveDate::from_ymd_opt(2023, 4, 6).unwrap()).to_value())
        );
        assert_eq!(
            dt_datetime.add_duration(Duration::days(1)),
            Some(DateTime::from(Utc.with_ymd_and_hms(2023, 4, 6, 12, 34, 56)).to_value())
        );
    }

    #[test]
    fn test_value_string_behavior() {
        let string = Value::from("hello");
        let concat = string.concat("!");
        assert!(concat == StringB::from("hello!").to_value())
    }

    #[test]
    fn test_value_as_string() {
        let string = Value::from("hello");
        assert!(string.as_string_b() == Some(&StringB::from("hello")))
    }

    #[test]
    fn test_value_as_number() {
        let number = Value::from(3.14);
        assert!(number.as_number() == Some(&Number::from(3.14)))
    }

    #[test]
    fn test_value_as_array_mut() {
        let mut array = Value::from(vec![1, 2, 3]);
        assert!(array.as_array_mut().unwrap().get_mut(0) == Some(&mut 1.to_value()))
    }

    #[test]
    fn test_value_as_object_mut() {
        let mut object = Value::from(HashMap::from_iter(vec![("1", 3.14.to_value())].into_iter()));
        assert!(object.as_object_mut().unwrap().get_mut("1") == Some(&mut 3.14.to_value()))
    }
}
