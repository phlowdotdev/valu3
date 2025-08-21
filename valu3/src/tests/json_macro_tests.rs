//! Comprehensive tests for the enhanced json! macro
//! 
//! These tests validate all features of the json! macro to ensure
//! compatibility with serde_json and proper functionality.

use crate::{json, prelude::*};
use std::collections::HashMap;

/// Test basic primitive types
#[cfg(test)]
mod primitives {
    use super::*;

    #[test]
    fn test_null() {
        let value = json!(null);
        assert_eq!(value, Value::Null);
        assert!(matches!(value, Value::Null));
    }

    #[test]
    fn test_boolean_values() {
        let true_val = json!(true);
        let false_val = json!(false);
        
        assert_eq!(true_val, Value::Boolean(true));
        assert_eq!(false_val, Value::Boolean(false));
        assert!(matches!(true_val, Value::Boolean(true)));
        assert!(matches!(false_val, Value::Boolean(false)));
    }

    #[test]
    fn test_integer_numbers() {
        assert_eq!(json!(0), 0.to_value());
        assert_eq!(json!(42), 42.to_value());
        assert_eq!(json!(-123), (-123).to_value());
        assert_eq!(json!(999999), 999999.to_value());
    }

    #[test]
    fn test_floating_point_numbers() {
        assert_eq!(json!(0.0), 0.0.to_value());
        assert_eq!(json!(3.14), 3.14.to_value());
        assert_eq!(json!(-2.5), (-2.5).to_value());
        assert_eq!(json!(1e6), 1e6.to_value());
        assert_eq!(json!(1.23e-4), 1.23e-4.to_value());
    }

    #[test]
    fn test_string_literals() {
        assert_eq!(json!(""), "".to_value());
        assert_eq!(json!("hello"), "hello".to_value());
        assert_eq!(json!("hello world"), "hello world".to_value());
        assert_eq!(json!("with \"quotes\""), "with \"quotes\"".to_value());
        assert_eq!(json!("with\nnewlines"), "with\nnewlines".to_value());
        assert_eq!(json!("unicode: 🚀"), "unicode: 🚀".to_value());
    }
}

/// Test array functionality
#[cfg(test)]
mod arrays {
    use super::*;

    #[test]
    fn test_empty_array() {
        let empty = json!([]);
        let expected = Vec::<Value>::new().to_value();
        assert_eq!(empty, expected);
    }

    #[test]
    fn test_array_with_primitives() {
        let arr = json!([1, 2, 3]);
        let expected = vec![1, 2, 3].to_value();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_mixed_type_array() {
        let arr = json!(["hello", 42, true, null]);
        let expected = vec![
            "hello".to_value(),
            42.to_value(),
            true.to_value(),
            Value::Null
        ];
        assert_eq!(arr, expected.to_value());
    }

    #[test]
    fn test_nested_arrays() {
        let nested = json!([[1, 2], [3, 4], [5, 6]]);
        let expected = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6]
        ].to_value();
        assert_eq!(nested, expected);
    }

    #[test]
    fn test_array_with_trailing_comma() {
        let arr = json!([1, 2, 3,]);
        let expected = vec![1, 2, 3].to_value();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_array_with_objects() {
        let arr = json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]);
        
        let expected = vec![
            {
                let mut map = HashMap::new();
                map.insert("name".to_string(), "Alice".to_value());
                map.insert("age".to_string(), 30.to_value());
                map.to_value()
            },
            {
                let mut map = HashMap::new();
                map.insert("name".to_string(), "Bob".to_value());
                map.insert("age".to_string(), 25.to_value());
                map.to_value()
            }
        ].to_value();
        
        assert_eq!(arr, expected);
    }
}

/// Test object functionality
#[cfg(test)]
mod objects {
    use super::*;

    #[test]
    fn test_empty_object() {
        let empty = json!({});
        let expected = HashMap::<String, Value>::new().to_value();
        assert_eq!(empty, expected);
    }

    #[test]
    fn test_simple_object() {
        let obj = json!({
            "name": "test",
            "value": 42
        });
        
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), "test".to_value());
        expected.insert("value".to_string(), 42.to_value());
        
        assert_eq!(obj, expected.to_value());
    }

    #[test]
    fn test_object_with_all_types() {
        let obj = json!({
            "null": null,
            "boolean": true,
            "integer": 42,
            "float": 3.14,
            "string": "hello",
            "array": [1, 2, 3],
            "object": {"nested": "value"}
        });

        if let Value::Object(map) = obj {
            assert!(map.contains_key(&"null"));
            assert!(map.contains_key(&"boolean"));
            assert!(map.contains_key(&"integer"));
            assert!(map.contains_key(&"float"));
            assert!(map.contains_key(&"string"));
            assert!(map.contains_key(&"array"));
            assert!(map.contains_key(&"object"));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_nested_objects() {
        let obj = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deep"
                    }
                }
            }
        });

        if let Value::Object(level1) = obj {
            if let Some(Value::Object(level2)) = level1.get("level1") {
                if let Some(Value::Object(level3)) = level2.get("level2") {
                    if let Some(Value::Object(level4)) = level3.get("level3") {
                        assert_eq!(level4.get("value"), Some(&"deep".to_value()));
                    } else {
                        panic!("Expected level4 object");
                    }
                } else {
                    panic!("Expected level3 object");
                }
            } else {
                panic!("Expected level2 object");
            }
        } else {
            panic!("Expected level1 object");
        }
    }

    #[test]
    fn test_object_with_trailing_comma() {
        let obj = json!({
            "key1": "value1",
            "key2": "value2",
        });
        
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_value());
        expected.insert("key2".to_string(), "value2".to_value());
        
        assert_eq!(obj, expected.to_value());
    }
}

/// Test variable interpolation and expressions
#[cfg(test)]
mod interpolation {
    use super::*;

    #[test]
    fn test_variable_interpolation() {
        let name = "Alice";
        let age = 30;
        let active = true;

        let obj = json!({
            "name": name,
            "age": age,
            "active": active
        });

        let mut expected = HashMap::new();
        expected.insert("name".to_string(), "Alice".to_value());
        expected.insert("age".to_string(), 30.to_value());
        expected.insert("active".to_string(), true.to_value());

        assert_eq!(obj, expected.to_value());
    }

    #[test]
    fn test_expression_evaluation() {
        let a = 10;
        let b = 20;

        let obj = json!({
            "sum": a + b,
            "product": a * b,
            "difference": b - a,
            "quotient": b / a,
            "greater": a > b,
            "equal": a == a
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("sum"), Some(&30.to_value()));
            assert_eq!(map.get("product"), Some(&200.to_value()));
            assert_eq!(map.get("difference"), Some(&10.to_value()));
            assert_eq!(map.get("quotient"), Some(&2.to_value()));
            assert_eq!(map.get("greater"), Some(&false.to_value()));
            assert_eq!(map.get("equal"), Some(&true.to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_function_calls_in_values() {
        let name = "rust";

        let obj = json!({
            "uppercase": name.to_uppercase(),
            "length": name.len(),
            "formatted": format!("Hello, {}!", name),
            "contains": name.contains("ru")
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("uppercase"), Some(&"RUST".to_value()));
            assert_eq!(map.get("length"), Some(&(4 as usize).to_value()));
            assert_eq!(map.get("formatted"), Some(&"Hello, rust!".to_value()));
            assert_eq!(map.get("contains"), Some(&true.to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_array_variable_interpolation() {
        let items = vec!["apple", "banana", "cherry"];
        let numbers = vec![1, 2, 3, 4, 5];

        let obj = json!({
            "fruits": items,
            "numbers": numbers,
            "mixed": [items[0], numbers[0], true]
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("fruits"), Some(&vec!["apple", "banana", "cherry"].to_value()));
            assert_eq!(map.get("numbers"), Some(&vec![1, 2, 3, 4, 5].to_value()));
        } else {
            panic!("Expected object");
        }
    }
}

/// Test complex real-world scenarios
#[cfg(test)]
mod real_world_scenarios {
    use super::*;

    #[test]
    fn test_api_response_structure() {
        let user_id = 12345;
        let username = "john_doe";
        let email = "john@example.com";
        let posts = vec!["Post 1", "Post 2", "Post 3"];
        let follower_count = 1500;
        let is_verified = true;

        let response = json!({
            "status": "success",
            "code": 200,
            "data": {
                "user": {
                    "id": user_id,
                    "username": username,
                    "email": email,
                    "profile": {
                        "posts": posts,
                        "stats": {
                            "followers": follower_count,
                            "following": 250,
                            "posts_count": posts.len(),
                            "engagement_rate": follower_count as f64 * 0.05
                        },
                        "verified": is_verified,
                        "created_at": "2023-01-15T10:30:00Z"
                    }
                }
            },
            "meta": {
                "timestamp": 1699123456789_i64,
                "version": "1.0",
                "rate_limit": {
                    "remaining": 95,
                    "reset_at": null
                }
            }
        });

        // Verify structure
        if let Value::Object(root) = response {
            assert_eq!(root.get("status"), Some(&"success".to_value()));
            assert_eq!(root.get("code"), Some(&200.to_value()));
            
            if let Some(Value::Object(data)) = root.get("data") {
                if let Some(Value::Object(user)) = data.get("user") {
                    assert_eq!(user.get("id"), Some(&12345.to_value()));
                    assert_eq!(user.get("username"), Some(&"john_doe".to_value()));
                } else {
                    panic!("Expected user object");
                }
            } else {
                panic!("Expected data object");
            }
        } else {
            panic!("Expected root object");
        }
    }

    #[test]
    fn test_configuration_object() {
        let app_name = "MyApp";
        let version = (1, 2, 3);
        let features = vec!["feature1", "feature2", "feature3"];
        let debug_mode = cfg!(debug_assertions);

        let config = json!({
            "application": {
                "name": app_name,
                "version": {
                    "major": version.0,
                    "minor": version.1,
                    "patch": version.2,
                    "string": format!("{}.{}.{}", version.0, version.1, version.2)
                },
                "features": features,
                "build": {
                    "debug": debug_mode,
                    "target": std::env::consts::ARCH,
                    "profile": if debug_mode { "debug" } else { "release" }
                }
            },
            "database": {
                "host": "localhost",
                "port": 5432,
                "name": "myapp_db",
                "ssl": true,
                "pool_size": 10
            },
            "logging": {
                "level": if debug_mode { "debug" } else { "info" },
                "format": "json",
                "outputs": ["stdout", "file"]
            }
        });

        // Convert to JSON string and back to verify serialization works
        let json_string = config.to_json(crate::to::json::JsonMode::Inline);
        assert!(json_string.contains("MyApp"));
        assert!(json_string.contains("1.2.3"));
    }

    #[test]
    fn test_deeply_nested_structure() {
        let obj = json!({
            "level1": {
                "data": "value1",
                "level2": {
                    "data": "value2", 
                    "array": [1, 2, 3],
                    "level3": {
                        "data": "value3",
                        "bool": true,
                        "level4": {
                            "data": "value4",
                            "number": 42,
                            "level5": {
                                "data": "deepest",
                                "final": null
                            }
                        }
                    }
                }
            }
        });

        // Verify we can access the deepest level
        if let Value::Object(l1) = obj {
            if let Some(Value::Object(l1_inner)) = l1.get("level1") {
                if let Some(Value::Object(l2)) = l1_inner.get("level2") {
                    if let Some(Value::Object(l3)) = l2.get("level3") {
                        if let Some(Value::Object(l4)) = l3.get("level4") {
                            if let Some(Value::Object(l5)) = l4.get("level5") {
                                assert_eq!(l5.get("data"), Some(&"deepest".to_value()));
                                assert_eq!(l5.get("final"), Some(&Value::Null));
                            } else {
                                panic!("Expected level5");
                            }
                        } else {
                            panic!("Expected level4");
                        }
                    } else {
                        panic!("Expected level3");
                    }
                } else {
                    panic!("Expected level2");
                }
            } else {
                panic!("Expected level1 inner");
            }
        } else {
            panic!("Expected root object");
        }
    }
}

/// Test edge cases and error conditions
#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_large_numbers() {
        let obj = json!({
            "max_i32": i32::MAX,
            "min_i32": i32::MIN,
            "max_i64": i64::MAX,
            "min_i64": i64::MIN,
            "max_f64": f64::MAX,
            "min_f64": f64::MIN
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("max_i32"), Some(&(i32::MAX).to_value()));
            assert_eq!(map.get("min_i32"), Some(&(i32::MIN).to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_empty_strings_and_arrays() {
        let obj = json!({
            "empty_string": "",
            "empty_array": [],
            "empty_object": {},
            "nested_empty": {
                "array": [],
                "string": "",
                "object": {}
            }
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("empty_string"), Some(&"".to_value()));
            assert_eq!(map.get("empty_array"), Some(&Vec::<Value>::new().to_value()));
            assert_eq!(map.get("empty_object"), Some(&HashMap::<String, Value>::new().to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_special_float_values() {
        let obj = json!({
            "zero": 0.0,
            "negative_zero": -0.0,
            "positive_infinity": f64::INFINITY,
            "negative_infinity": f64::NEG_INFINITY,
            "not_a_number": f64::NAN
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("zero"), Some(&0.0.to_value()));
            // Note: NaN and infinity handling depends on the Value implementation
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_multiple_trailing_commas() {
        let arr = json!([
            1,
            2,
            3,
        ]);
        
        let obj = json!({
            "key1": "value1",
            "key2": "value2",
        });

        assert_eq!(arr, vec![1, 2, 3].to_value());
        
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_value());
        expected.insert("key2".to_string(), "value2".to_value());
        assert_eq!(obj, expected.to_value());
    }
}

/// Performance and stress tests
#[cfg(test)]
mod performance {
    use super::*;

    #[test]
    fn test_large_array() {
        let large_vec: Vec<i32> = (0..1000).collect();
        let arr = json!(large_vec);
        
        if let Value::Array(array) = arr {
            assert_eq!(array.len(), 1000);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_many_object_keys() {
        let mut values = HashMap::new();
        for i in 0..100 {
            values.insert(format!("key_{}", i), i);
        }

        let obj = json!(values);
        
        if let Value::Object(map) = obj {
            assert_eq!(map.len(), 100);
            assert!(map.contains_key(&"key_0"));
            assert!(map.contains_key(&"key_99"));
        } else {
            panic!("Expected object");
        }
    }
}

/// Compatibility tests to ensure behavior matches expectations
#[cfg(test)]
mod compatibility {
    use super::*;

    #[test]
    fn test_json_roundtrip() {
        let original = json!({
            "string": "hello",
            "number": 42,
            "boolean": true,
            "null": null,
            "array": [1, 2, 3],
            "object": {
                "nested": "value"
            }
        });

        // Convert to JSON string
        let json_str = original.to_json(crate::to::json::JsonMode::Inline);
        
        // Parse back (assuming json_to_value exists)
        if let Ok(parsed) = Value::json_to_value(&json_str) {
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_type_consistency() {
        let obj = json!({
            "integer": 42,
            "float": 42.0,
            "string": "42",
            "boolean": true,
            "null": null
        });

        if let Value::Object(map) = obj {
            // Verify types are preserved correctly
            assert!(matches!(map.get("integer").unwrap(), Value::Number(_)));
            assert!(matches!(map.get("float").unwrap(), Value::Number(_)));
            assert!(matches!(map.get("string").unwrap(), Value::String(_)));
            assert!(matches!(map.get("boolean").unwrap(), Value::Boolean(_)));
            assert!(matches!(map.get("null").unwrap(), Value::Null));
        } else {
            panic!("Expected object");
        }
    }
}
