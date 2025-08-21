// tests/derive_tests.rs
use std::collections::HashMap;
use valu3::prelude::*;

#[derive(ToValue, FromValue, ToJson)]
struct User {
    id: u32,
    name: String,
    status: Status,
}

#[derive(Clone, ToValue, FromValue, ToJson, PartialEq, Debug)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn test_to_value_struct() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        status: Status::Active,
    };
    let value = user.to_value();

    if let Value::Object(map) = value {
        assert_eq!(map.get("id").unwrap(), &Value::from(1u32));
        assert_eq!(map.get("name").unwrap(), &Value::from("Alice"));
        assert_eq!(map.get("status").unwrap(), &Value::from("Active"));
    } else {
        panic!("Expected Value::Object");
    }
}

#[test]
fn test_from_value_struct() {
    let mut map = HashMap::new();
    map.insert("id".to_string(), Value::from(1u32));
    map.insert("name".to_string(), Value::from("Alice"));
    map.insert("status".to_string(), Value::from("Active"));
    let value = map.to_value();

    let user = User::from_value(value).expect("Should parse correctly");

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice".to_string());
    assert_eq!(user.status, Status::Active);
}

#[test]
fn test_to_value_enum() {
    let status = Status::Active;
    let value = status.to_value();

    assert_eq!(value, Value::from("Active"));
}

#[test]
fn test_from_value_enum() {
    let value = Value::from("Active");
    let status = Status::from_value(value).expect("Should parse correctly");

    match status {
        Status::Active => (),
        _ => panic!("Expected Status::Active"),
    }
}

#[test]
fn test_to_json() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        status: Status::Active,
    };
    let json = user.to_json();
    let expect = {
        let mut map = HashMap::new();
        map.insert("id".to_string(), Value::from(1));
        map.insert("name".to_string(), Value::from("Alice"));
        map.insert("status".to_string(), Value::from("Active"));
        map.to_value()
    };
    let result = Value::json_to_value(&json).unwrap();

    assert_eq!(result, expect);
}
