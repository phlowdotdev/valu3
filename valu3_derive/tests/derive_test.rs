// tests/derive_tests.rs
use std::collections::HashMap;
use valu3::prelude::*;
use valu3_derive::*;

#[derive(ToValue, FromValue, ToJson)]
struct User {
    id: u32,
    name: String,
}

#[derive(ToValue, FromValue, ToJson)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn test_to_value_struct() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };
    let value = user.to_value();

    if let Value::Object(map) = value {
        assert_eq!(map.get("id").unwrap(), &Value::from(1u32));
        assert_eq!(map.get("name").unwrap(), &Value::from("Alice"));
    } else {
        panic!("Expected Value::Object");
    }
}

#[test]
fn test_from_value_struct() {
    let mut map = HashMap::new();
    map.insert("id".to_string(), Value::from(1));
    map.insert("name".to_string(), Value::from("Alice"));

    let user = User::from_value(map.to_value()).expect("Should parse correctly");

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice".to_string());
}

// #[test]
// fn test_to_value_enum() {
//     let status = Status::Active;
//     let value = status.to_value();

//     assert_eq!(value, Value::from("Active"));
// }

// #[test]
// fn test_from_value_enum() {
//     let value = Value::from("Active");
//     let status = Status::from_value(value).expect("Should parse correctly");

//     match status {
//         Status::Active => (),
//         _ => panic!("Expected Status::Active"),
//     }
// }

// #[test]
// fn test_to_json() {
//     let user = User {
//         id: 1,
//         name: "Alice".to_string(),
//     };
//     let json = user.to_json();
//     assert_eq!(json, r#"{"id":1,"name":"Alice"}"#);
// }

// #[test]
// fn test_to_yaml() {
//     let user = User {
//         id: 1,
//         name: "Alice".to_string(),
//     };
//     let yaml = user.to_yaml();

//     let expected_yaml = "---\nid: 1\nname: Alice\n";
//     assert_eq!(yaml.trim(), expected_yaml.trim());
// }
