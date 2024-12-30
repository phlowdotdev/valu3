pub mod de;
pub mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

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
            r#"{"b":2,"a":1,"c":3}"#,
            r#"{"b":2,"c":3,"a":1}"#,
            r#"{"c":3,"b":2,"a":1}"#,
            r#"{"c":3,"a":1,"b":2}"#,
        ];
        println!("{}", serialized);
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
