// Simple json! macro for basic JSON-like syntax
// Simple json! macro for basic JSON-like syntax
// Note: This is a simplified version and doesn't support all JSON features
#[macro_export]
macro_rules! json {
    // Handle null
    (null) => {
        $crate::value::Value::Null
    };

    // Handle booleans
    (true) => {
        $crate::value::Value::from(true)
    };
    (false) => {
        $crate::value::Value::from(false)
    };

    // Handle empty object
    ({}) => {{
        use std::collections::HashMap;
        use $crate::traits::ToValueBehavior;
        HashMap::<String, $crate::value::Value>::new().to_value()
    }};

    // Handle object with key-value pairs (values that are token-trees — e.g. nested objects/arrays)
    ({ $($key:tt : $value:tt),* $(,)? }) => {{
        use std::collections::HashMap;
        use $crate::traits::ToValueBehavior;
        let mut map = HashMap::new();
        $(
            map.insert($key.to_string(), json!($value));
        )*
        map.to_value()
    }};

    // Fallback: values that are expressions
    ({ $($key:tt : $value:expr),* $(,)? }) => {{
        use std::collections::HashMap;
        use $crate::traits::ToValueBehavior;
        let mut map = HashMap::new();
        $(
            map.insert($key.to_string(), json!($value));
        )*
        map.to_value()
    }};

    // Handle empty array
    ([]) => {{
        use $crate::traits::ToValueBehavior;
        Vec::<$crate::value::Value>::new().to_value()
    }};

    // Handle array with values (token-tree entries first, to allow nested arrays/objects)
    ([ $($value:tt),* $(,)? ]) => {{
        use $crate::traits::ToValueBehavior;
        vec![ $(json!($value)),* ].to_value()
    }};

    // Fallback: array entries as expressions
    ([ $($value:expr),* $(,)? ]) => {{
        use $crate::traits::ToValueBehavior;
        vec![ $(json!($value)),* ].to_value()
    }};

    // Handle any other expression
    ($val:expr) => {{
        use $crate::traits::ToValueBehavior;
        ($val).to_value()
    }};
}

#[cfg(test)]
mod test {
    use crate::{prelude::JsonMode, traits::ToValueBehavior};
    use std::collections::HashMap;

    #[test]
    fn test_json_null() {
        let null = json!(null);
        assert_eq!(null, crate::value::Value::Null);
    }

    #[test]
    fn test_json_booleans() {
        let bool_true = json!(true);
        assert_eq!(bool_true, crate::value::Value::from(true));

        let bool_false = json!(false);
        assert_eq!(bool_false, crate::value::Value::from(false));
    }

    #[test]
    fn test_json_number() {
        let number = json!(42);
        assert_eq!(number, 42.to_value());
    }

    #[test]
    fn test_json_string() {
        let string = json!("hello");
        assert_eq!(string, "hello".to_value());
    }

    #[test]
    fn test_json_array() {
        let array = json!(vec![1, 2, 3]);
        assert_eq!(array, vec![1, 2, 3].to_value());
    }

    #[test]
    fn test_json_variable() {
        let test_val = true;
        let from_var = json!(test_val);
        assert_eq!(from_var, test_val.to_value());
    }

    #[test]
    fn test_json_complex_structure() {
        let mut map = HashMap::new();
        map.insert("test".to_string(), true.to_value());
        map.insert("test2".to_string(), "ok".to_value());
        map.insert("test3".to_string(), vec![0, 1].to_value());

        let complex = json!(map);
        assert_eq!(complex, map.to_value());
    }

    #[test]
    fn test_json_declarative() {
        let data = json!({
            "name": "test",
            "value": 42,
            "active": true
        });

        let expected = {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "test".to_value());
            map.insert("value".to_string(), 42.to_value());
            map.insert("active".to_string(), true.to_value());
            map.to_value()
        };

        assert_eq!(data, expected);
    }

    #[test]
    fn test_json_declarative_with_expressions() {
        let active = true;
        let value = "42";
        let data = json!({
            "name": "test",
            "value": value.parse::<i32>().unwrap(),
            "active": active
        });

        let expected = {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "test".to_value());
            map.insert("value".to_string(), 42.to_value());
            map.insert("active".to_string(), true.to_value());
            map.to_value()
        };

        assert_eq!(data, expected);
    }

    #[test]
    fn test_json_array_declarative() {
        let value = "42";
        let data = json!([1, "hello", true, value.parse::<i32>().unwrap()]);

        let expected = vec![
            1.to_value(),
            "hello".to_value(),
            true.to_value(),
            42.to_value(),
        ]
        .to_value();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_json_empty_containers() {
        let empty_obj = json!({});
        let empty_arr = json!([]);

        assert_eq!(
            empty_obj,
            HashMap::<String, crate::value::Value>::new().to_value()
        );
        assert_eq!(empty_arr, Vec::<crate::value::Value>::new().to_value());
    }

    #[test]
    fn test_json_nested_structures() {
        let data = json!({
            "user": {
                "id": 1,
                "name": "Alice",
                "roles": ["admin", "user"]
            },
            "active": true,
            "score": 99.5
        });

        let mut user_map = HashMap::new();
        user_map.insert("id".to_string(), 1.to_value());
        user_map.insert("name".to_string(), "Alice".to_value());
        user_map.insert(
            "roles".to_string(),
            vec!["admin".to_value(), "user".to_value()].to_value(),
        );

        let mut expected_map = HashMap::new();
        expected_map.insert("user".to_string(), user_map.to_value());
        expected_map.insert("active".to_string(), true.to_value());
        expected_map.insert("score".to_string(), 99.5.to_value());

        assert_eq!(data, expected_map.to_value());
    }

    #[test]
    fn test_recursive_json() {
        let inner = json!({
                "level2": r#"<img src="level2.png" />"#
        });
        let data = json!({
            "level1": inner
        });

        let mut level2_map = HashMap::new();
        level2_map.insert(
            "level2".to_string(),
            r#"<img src="level2.png" />"#.to_value(),
        );

        let mut expected_map = HashMap::new();
        expected_map.insert("level1".to_string(), level2_map.to_value());
        assert_eq!(data, expected_map.to_value());

        let string_json = data.to_json(JsonMode::Inline);
        let expected_json = r#"{"level1": {"level2": "<img src=\"level2.png\" />"}}"#;

        assert_eq!(string_json, expected_json);
    }
}
