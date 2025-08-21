//! Benchmark tests for the json! macro performance
//! 
//! These tests measure the performance characteristics of the json! macro
//! and compare with manual construction methods.

use crate::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

#[cfg(test)]
mod benchmarks {
    use crate::json;
    use super::*;

    #[test]
    fn benchmark_json_macro_vs_manual_simple() {
        let iterations = 1000;
        
        // Benchmark json! macro
        let start = Instant::now();
        for _ in 0..iterations {
            let _value = json!({
                "name": "test",
                "value": 42,
                "active": true
            });
        }
        let macro_duration = start.elapsed();
        
        // Benchmark manual construction
        let start = Instant::now();
        for _ in 0..iterations {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "test".to_value());
            map.insert("value".to_string(), 42.to_value());
            map.insert("active".to_string(), true.to_value());
            let _value = map.to_value();
        }
        let manual_duration = start.elapsed();
        
        println!("Simple object creation:");
        println!("  json! macro: {:?}", macro_duration);
        println!("  Manual:      {:?}", manual_duration);
        println!("  Ratio:       {:.2}x", 
                 manual_duration.as_nanos() as f64 / macro_duration.as_nanos() as f64);
    }

    #[test]
    fn benchmark_json_macro_vs_manual_complex() {
        let iterations = 100;
        
        // Benchmark json! macro
        let start = Instant::now();
        for _ in 0..iterations {
            let _value = json!({
                "user": {
                    "id": 123,
                    "name": "John Doe",
                    "email": "john@example.com",
                    "profile": {
                        "age": 30,
                        "interests": ["programming", "music", "travel"],
                        "settings": {
                            "theme": "dark",
                            "notifications": true,
                            "language": "en"
                        }
                    }
                },
                "meta": {
                    "created": "2023-01-01",
                    "version": 1
                }
            });
        }
        let macro_duration = start.elapsed();
        
        // Benchmark manual construction
        let start = Instant::now();
        for _ in 0..iterations {
            let mut root = HashMap::new();
            
            let mut user = HashMap::new();
            user.insert("id".to_string(), 123.to_value());
            user.insert("name".to_string(), "John Doe".to_value());
            user.insert("email".to_string(), "john@example.com".to_value());
            
            let mut profile = HashMap::new();
            profile.insert("age".to_string(), 30.to_value());
            profile.insert("interests".to_string(), 
                          vec!["programming", "music", "travel"].to_value());
            
            let mut settings = HashMap::new();
            settings.insert("theme".to_string(), "dark".to_value());
            settings.insert("notifications".to_string(), true.to_value());
            settings.insert("language".to_string(), "en".to_value());
            
            profile.insert("settings".to_string(), settings.to_value());
            user.insert("profile".to_string(), profile.to_value());
            root.insert("user".to_string(), user.to_value());
            
            let mut meta = HashMap::new();
            meta.insert("created".to_string(), "2023-01-01".to_value());
            meta.insert("version".to_string(), 1.to_value());
            root.insert("meta".to_string(), meta.to_value());
            
            let _value = root.to_value();
        }
        let manual_duration = start.elapsed();
        
        println!("Complex object creation:");
        println!("  json! macro: {:?}", macro_duration);
        println!("  Manual:      {:?}", manual_duration);
        println!("  Ratio:       {:.2}x", 
                 manual_duration.as_nanos() as f64 / macro_duration.as_nanos() as f64);
    }

    #[test]
    fn benchmark_large_array_creation() {
        let iterations = 100;
        let size = 1000;
        
        // Benchmark json! macro
        let start = Instant::now();
        for _ in 0..iterations {
            let numbers: Vec<i32> = (0..size).collect();
            let _value = json!(numbers);
        }
        let macro_duration = start.elapsed();
        
        // Benchmark manual construction
        let start = Instant::now();
        for _ in 0..iterations {
            let numbers: Vec<i32> = (0..size).collect();
            let _value = numbers.to_value();
        }
        let manual_duration = start.elapsed();
        
        println!("Large array ({} elements) creation:", size);
        println!("  json! macro: {:?}", macro_duration);
        println!("  Manual:      {:?}", manual_duration);
        println!("  Ratio:       {:.2}x", 
                 manual_duration.as_nanos() as f64 / macro_duration.as_nanos() as f64);
    }

    #[test]
    fn benchmark_variable_interpolation() {
        let iterations = 1000;
        let name = "benchmark_test";
        let count = 42;
        let active = true;
        let tags = vec!["rust", "json", "macro"];
        
        // Benchmark json! macro with variables
        let start = Instant::now();
        for _ in 0..iterations {
            let _value = json!({
                "name": name,
                "count": count,
                "active": active,
                "tags": tags.clone(),
                "computed": count * 2,
                "formatted": format!("Item: {}", name)
            });
        }
        let macro_duration = start.elapsed();
        
        // Benchmark manual construction with variables
        let start = Instant::now();
        for _ in 0..iterations {
            let mut map = HashMap::new();
            map.insert("name".to_string(), name.to_value());
            map.insert("count".to_string(), count.to_value());
            map.insert("active".to_string(), active.to_value());
            map.insert("tags".to_string(), tags.clone().to_value());
            map.insert("computed".to_string(), (count * 2).to_value());
            map.insert("formatted".to_string(), format!("Item: {}", name).to_value());
            let _value = map.to_value();
        }
        let manual_duration = start.elapsed();
        
        println!("Variable interpolation:");
        println!("  json! macro: {:?}", macro_duration);
        println!("  Manual:      {:?}", manual_duration);
        println!("  Ratio:       {:.2}x", 
                 manual_duration.as_nanos() as f64 / macro_duration.as_nanos() as f64);
    }

    #[test]
    fn stress_test_deeply_nested() {
        // Test with deep nesting to ensure no stack overflow
        let depth = 50;
        
        let start = Instant::now();
        let mut current = json!("deep_value");
        
        for i in 0..depth {
            current = json!({
                format!("level_{}", i): current
            });
        }
        let duration = start.elapsed();
        
        println!("Deep nesting ({} levels): {:?}", depth, duration);
        
        // Verify structure is correct
        let mut current_ref = &current;
        for i in (0..depth).rev() {
            if let Value::Object(map) = current_ref {
                let key = format!("level_{}", i);
                current_ref = map.get(key).expect("Missing nested level");
            } else {
                panic!("Expected object at level {}", i);
            }
        }
        
        assert_eq!(*current_ref, "deep_value".to_value());
    }

    #[test]
    fn stress_test_wide_object() {
        // Test with many keys to test hash map performance
        let key_count = 1000;
        
        let start = Instant::now();
        
        // Create a large object using variables
        let mut values = HashMap::new();
        for i in 0..key_count {
            values.insert(format!("key_{}", i), format!("value_{}", i));
        }
        
        let obj = json!(values);
        let creation_duration = start.elapsed();
        
        // Test access performance
        let start = Instant::now();
        if let Value::Object(map) = &obj {
            for i in 0..key_count {
                let key = format!("key_{}", i);
                assert!(map.contains_key(&key));
            }
        }
        let access_duration = start.elapsed();
        
        println!("Wide object ({} keys):", key_count);
        println!("  Creation: {:?}", creation_duration);
        println!("  Access:   {:?}", access_duration);
    }

    #[test]
    fn memory_usage_estimation() {
        use std::mem;
        
        // Estimate memory usage of different Value types
        println!("Memory usage estimation:");
        println!("  Value enum: {} bytes", mem::size_of::<Value>());
        println!("  String: {} bytes", mem::size_of::<String>());
        println!("  i32: {} bytes", mem::size_of::<i32>());
        println!("  f64: {} bytes", mem::size_of::<f64>());
        println!("  bool: {} bytes", mem::size_of::<bool>());
        
        // Test memory efficiency with different json! patterns
        let simple = json!(42);
        let string = json!("hello world");
        let array = json!([1, 2, 3, 4, 5]);
        let object = json!({"key": "value", "number": 42});
        
        println!("  Simple number: ~{} bytes", mem::size_of_val(&simple));
        println!("  String value: ~{} bytes", mem::size_of_val(&string));
        println!("  Array value: ~{} bytes", mem::size_of_val(&array));
        println!("  Object value: ~{} bytes", mem::size_of_val(&object));
    }
}

#[cfg(test)]
mod correctness_stress_tests {
    use crate::json;
    use super::*;

    #[test]
    fn test_macro_with_many_variables() {
        // Test macro with many variable interpolations
        let vars: Vec<i32> = (0..100).collect();
        
        let obj = json!({
            "var_0": vars[0],
            "var_10": vars[10],
            "var_25": vars[25],
            "var_50": vars[50],
            "var_75": vars[75],
            "var_99": vars[99],
            "sum": vars.iter().sum::<i32>(),
            "len": vars.len(),
            "first_ten": vars[0..10].to_vec(),
            "last_ten": vars[90..100].to_vec()
        });
        
        if let Value::Object(map) = obj {
            assert_eq!(map.get("var_0"), Some(&0.to_value()));
            assert_eq!(map.get("var_99"), Some(&99.to_value()));
            assert_eq!(map.get("sum"), Some(&4950.to_value())); // Sum of 0..100
            assert_eq!(map.get("len"), Some(&100.to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_macro_with_complex_expressions() {
        let base: i32 = 10;
        let multiplier = 3;
        let items = vec!["a", "b", "c"];
        
        let obj = json!({
            "math": {
                "pow": base.pow(2),
                "sqrt": (base as f64).sqrt(),
                "complex": (base * multiplier + 5) / 2
            },
            "strings": {
                "joined": items.join(","),
                "uppercase": items.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>(),
                "filtered": items.iter().filter(|&&s| s != "b").cloned().collect::<Vec<_>>()
            },
            "conditionals": {
                "is_even": base % 2 == 0,
                "max_value": if base > 5 { "high" } else { "low" },
                "category": match base {
                    1..=5 => "small",
                    6..=10 => "medium",
                    _ => "large"
                }
            }
        });
        
        if let Value::Object(root) = obj {
            if let Some(Value::Object(math)) = root.get("math") {
                assert_eq!(math.get("pow"), Some(&100.to_value()));
                assert_eq!(math.get("complex"), Some(&22.to_value()));
            } else {
                panic!("Expected math object");
            }
            
            if let Some(Value::Object(strings)) = root.get("strings") {
                assert_eq!(strings.get("joined"), Some(&"a,b,c".to_value()));
            } else {
                panic!("Expected strings object");
            }
            
            if let Some(Value::Object(conditionals)) = root.get("conditionals") {
                assert_eq!(conditionals.get("is_even"), Some(&true.to_value()));
                assert_eq!(conditionals.get("category"), Some(&"medium".to_value()));
            } else {
                panic!("Expected conditionals object");
            }
        } else {
            panic!("Expected root object");
        }
    }

    #[test]
    fn test_macro_error_resistance() {
        // Test that the macro works with edge case values
        let nan = f64::NAN;
        let infinity = f64::INFINITY;
        let empty_string = "";
        let empty_vec: Vec<i32> = vec![];
        let empty_map: HashMap<String, i32> = HashMap::new();
        
        let obj = json!({
            "nan": nan,
            "infinity": infinity,
            "negative_infinity": f64::NEG_INFINITY,
            "empty_string": empty_string,
            "empty_vec": empty_vec,
            "empty_map": empty_map,
            "zero": 0,
            "negative_zero": -0.0
        });
        
        // Should not panic and should create a valid object
        assert!(matches!(obj, Value::Object(_)));
        
        if let Value::Object(map) = obj {
            assert!(map.contains_key(&"nan"));
            assert!(map.contains_key(&"infinity"));
            assert!(map.contains_key(&"empty_string"));
            assert_eq!(map.get("zero"), Some(&0.to_value()));
        } else {
            panic!("Expected object");
        }
    }
}

#[cfg(test)]
mod compilation_tests {
    use crate::json;
    use super::*;

    #[test]
    fn test_compile_time_evaluation() {
        // Test that constant expressions are evaluated at compile time
        const COMPILE_TIME_VALUE: i32 = 42 * 2;
        
        let obj = json!({
            "constant": COMPILE_TIME_VALUE,
            "computed_constant": 10 + 5,
            "string_constant": concat!("hello", " ", "world")
        });
        
        if let Value::Object(map) = obj {
            assert_eq!(map.get("constant"), Some(&84.to_value()));
            assert_eq!(map.get("computed_constant"), Some(&15.to_value()));
            assert_eq!(map.get("string_constant"), Some(&"hello world".to_value()));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_type_inference() {
        // Test that the macro correctly infers types
        let obj = json!({
            "auto_int": 42,
            "auto_float": 3.14,
            "auto_string": "hello",
            "auto_bool": true,
            "auto_array": [1, 2, 3],
            "auto_null": null
        });
        
        if let Value::Object(map) = obj {
            assert!(matches!(map.get("auto_int").unwrap(), Value::Number(_)));
            assert!(matches!(map.get("auto_float").unwrap(), Value::Number(_)));
            assert!(matches!(map.get("auto_string").unwrap(), Value::String(_)));
            assert!(matches!(map.get("auto_bool").unwrap(), Value::Boolean(_)));
            assert!(matches!(map.get("auto_array").unwrap(), Value::Array(_)));
            assert!(matches!(map.get("auto_null").unwrap(), Value::Null));
        } else {
            panic!("Expected object");
        }
    }
}
