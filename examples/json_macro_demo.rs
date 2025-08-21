use valu3::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("🚀 Valu3 - Value type and json! macro demo");
    println!("==========================================");

    // Basic types
    println!("\n📝 Basic Types:");
    let null_val = Value::Null;
    let bool_true = true.to_value();
    let bool_false = false.to_value();
    let number = 42.to_value();
    let float_num = 3.14159.to_value();
    let text = "Hello, Valu3!".to_value();

    println!("null: {}", null_val);
    println!("boolean: {}", bool_true);
    println!("boolean: {}", bool_false);
    println!("integer: {}", number);
    println!("float: {}", float_num);
    println!("string: {}", text);

    // Arrays
    println!("\n📋 Arrays:");
    let empty_array = Vec::<Value>::new().to_value();
    let number_array = vec![1, 2, 3, 4, 5].to_value();
    let mixed_array = vec![
        "hello".to_value(),
        42.to_value(),
        true.to_value(),
        Value::Null,
    ].to_value();
    let nested_array = vec![
        vec![1, 2].to_value(),
        vec![3, 4].to_value(),
        vec![5, 6].to_value(),
    ].to_value();

    println!("empty array: {}", empty_array);
    println!("numbers: {}", number_array);
    println!("mixed types: {}", mixed_array);
    println!("nested: {}", nested_array);

    // Objects
    println!("\n🗂️  Objects:");
    let empty_obj = HashMap::<String, Value>::new().to_value();
    
    let mut simple_map = HashMap::new();
    simple_map.insert("name".to_string(), "Valu3".to_value());
    simple_map.insert("version".to_string(), "0.8.3".to_value());
    simple_map.insert("type".to_string(), "library".to_value());
    let simple_obj = simple_map.to_value();

    println!("empty object: {}", empty_obj);
    println!("simple object: {}", simple_obj);

    // Complex structures with variables
    println!("\n🔧 Complex Structures with Variables:");
    let app_name = "MyApp";
    let version_major = 1;
    let version_minor = 2;
    let features = vec!["json", "serde", "parser"];
    let is_stable = true;

    // Build version object
    let mut version_obj = HashMap::new();
    version_obj.insert("major".to_string(), version_major.to_value());
    version_obj.insert("minor".to_string(), version_minor.to_value());
    version_obj.insert("full".to_string(), format!("{}.{}", version_major, version_minor).to_value());

    // Build metadata object
    let mut metadata_obj = HashMap::new();
    metadata_obj.insert("stable".to_string(), is_stable.to_value());
    metadata_obj.insert("license".to_string(), "Apache-2.0".to_value());
    metadata_obj.insert("author".to_string(), "Philippe Assis".to_value());

    // Build application object
    let mut app_obj = HashMap::new();
    app_obj.insert("name".to_string(), app_name.to_value());
    app_obj.insert("version".to_string(), version_obj.to_value());
    app_obj.insert("features".to_string(), features.to_value());
    app_obj.insert("metadata".to_string(), metadata_obj.to_value());

    // Build dependencies array
    let dependencies = vec![
        vec![("name", "serde"), ("version", "1.0")].to_value(),
        vec![("name", "chrono"), ("version", "0.4")].to_value(),
        vec![("name", "pest"), ("version", "2.7")].to_value(),
    ];

    // Build build object
    let mut build_obj = HashMap::new();
    build_obj.insert("timestamp".to_string(), 1699123456.to_value());
    build_obj.insert("environment".to_string(), "development".to_value());
    build_obj.insert("dependencies".to_string(), dependencies.to_value());

    // Build main config
    let mut config = HashMap::new();
    config.insert("application".to_string(), app_obj.to_value());
    config.insert("build".to_string(), build_obj.to_value());
    let app_config = config.to_value();

    println!("complex config: {}", app_config);

    // Arrays with different syntaxes
    println!("\n✨ Different Array Creation Methods:");
    let array1 = vec!["first", "second", "third"].to_value();
    let array2 = Array::from(vec!["first".to_value(), "second".to_value(), "third".to_value()]);
    
    println!("array method 1: {}", array1);
    println!("array method 2: {}", Value::Array(array2));

    // Objects with different syntaxes
    println!("\n🗂️ Different Object Creation Methods:");
    let mut obj1 = HashMap::new();
    obj1.insert("key1".to_string(), "value1".to_value());
    obj1.insert("key2".to_string(), "value2".to_value());
    obj1.insert("key3".to_string(), "value3".to_value());
    
    let obj2 = vec![
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3"),
    ].to_value();

    println!("object method 1: {}", obj1.to_value());
    println!("object method 2: {}", obj2);

    // Dynamic keys and computed values
    println!("\n🧮 Dynamic Keys and Computed Values:");
    let counter = 5;
    
    let mut dynamic_map = HashMap::new();
    dynamic_map.insert("static_key".to_string(), "static_value".to_value());
    dynamic_map.insert("computed_sum".to_string(), (2 + 3).to_value());
    dynamic_map.insert("computed_product".to_string(), (counter * 10).to_value());
    dynamic_map.insert("boolean_logic".to_string(), (counter > 3).to_value());
    dynamic_map.insert("string_interpolation".to_string(), format!("Counter is {}", counter).to_value());
    let dynamic_obj = dynamic_map.to_value();

    println!("dynamic object: {}", dynamic_obj);

    // Real-world API response example
    println!("\n🌐 Real-world API Response Example:");
    let user_id = 12345;
    let username = "john_doe";
    let posts = vec!["First post", "Second post", "Third post"];
    let follower_count = 1337;

    // Build stats object
    let mut stats = HashMap::new();
    stats.insert("followers".to_string(), follower_count.to_value());
    stats.insert("following".to_string(), 42.to_value());
    stats.insert("posts_count".to_string(), 156.to_value());

    // Build profile object
    let mut profile = HashMap::new();
    profile.insert("posts".to_string(), posts.to_value());
    profile.insert("stats".to_string(), stats.to_value());
    profile.insert("verified".to_string(), true.to_value());
    profile.insert("joined".to_string(), "2023-01-15T10:30:00Z".to_value());

    // Build user object
    let mut user = HashMap::new();
    user.insert("id".to_string(), user_id.to_value());
    user.insert("username".to_string(), username.to_value());
    user.insert("profile".to_string(), profile.to_value());

    // Build data object
    let mut data = HashMap::new();
    data.insert("user".to_string(), user.to_value());

    // Build rate_limit object
    let mut rate_limit = HashMap::new();
    rate_limit.insert("remaining".to_string(), 95.to_value());
    rate_limit.insert("reset".to_string(), Value::Null);

    // Build meta object
    let mut meta = HashMap::new();
    meta.insert("timestamp".to_string(), 1699123456789_i64.to_value());
    meta.insert("version".to_string(), "v1.0".to_value());
    meta.insert("rate_limit".to_string(), rate_limit.to_value());

    // Build final response
    let mut response = HashMap::new();
    response.insert("status".to_string(), "success".to_value());
    response.insert("data".to_string(), data.to_value());
    response.insert("meta".to_string(), meta.to_value());
    let api_response = response.to_value();

    println!("API response: {}", api_response);

    // Converting to JSON string
    println!("\n📤 JSON String Output:");
    println!("{}", api_response.to_json(JsonMode::Indented));

    // Using the json! macro for simple cases
    println!("\n🎯 Using json! macro for simple values:");
    println!("null: {}", json!(null));
    println!("true: {}", json!(true));
    println!("false: {}", json!(false));
    println!("number: {}", json!(42));
    println!("string: {}", json!("test"));
    println!("from variable: {}", json!(counter));

    println!("\n✅ All examples completed successfully!");
    println!("Valu3 provides flexible ways to create and manipulate Values!");
}
