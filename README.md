# Valu3: Unleash the Power of Data Manipulation in Rust 🚀

Welcome to **Valu3** - the ultimate, flexible, and powerful library for manipulating diverse data types in your Rust projects. Say goodbye to the complexity of handling numbers, strings, arrays, objects, and datetime values. Valu3 is here to make your life easier!


[![crates.io](https://img.shields.io/crates/v/valu3?label=0.6.4)](https://crates.io/crates/valu3)
[![Documentation](https://docs.rs/valu3/badge.svg?version=0.6.4)](https://docs.rs/valu3/0.6.4)
![MSRV](https://img.shields.io/badge/rustc-1.59+-ab6000.svg)
![Apache 2.0 licensed](https://img.shields.io/crates/l/actix-web.svg)
[![Dependency Status](https://deps.rs/crate/valu3/0.6.4/status.svg)](https://deps.rs/crate/valu3/0.6.4)
![Main test](https://github.com/lowcarboncode/valu3/actions/workflows/main-test.yml/badge.svg)
[![codecov](https://codecov.io/gh/lowcarboncode/valu3/branch/master/graph/badge.svg)](https://codecov.io/gh/lowcarboncode/valu3)
![downloads](https://img.shields.io/crates/d/valu3.svg)

## 🌟 Key Features

1. **Universal Type Handling**: No more juggling with different data types! Valu3's generic type support enables smooth management of all data types under one roof.
2. **Intuitive API**: Experience effortless data manipulation with Valu3's user-friendly API, designed to provide consistency across diverse data types.
3. **All-in-One Data Manipulation**: Valu3 has it all - numeric, string, object, array, and date/time manipulation. No need to look anywhere else!
4. **Effortless Data Conversion**: Convert your data to and from popular formats like JSON, YAML, and XML with ease, thanks to Valu3's built-in support.
5. **Serde Integration**: Serialize and deserialize your data seamlessly with Valu3's out-of-the-box integration with the Serde library.
6. **Native Struct Parsing & Validation**: Valu3 and Pest join forces to offer native parsing, conversion, and validation of data to structs, ensuring data integrity at every step.
7. **Payload Interpretation & Transformation**: Valu3 interprets and transforms payload strings like a champ, handling JSON from HTTP request bodies and more.
8. **Serde Converters**: Prepare to use Serde converters for enhanced data serialization and deserialization capabilities.

## 💡 Why Choose Valu3?

Valu3 is designed to make data manipulation tasks in Rust a breeze. By combining a wide range of features and a consistent API, it simplifies data handling in Rust projects while maximizing productivity.

Join the Valu3 revolution and experience the future of data manipulation in Rust! 🎉

**⚡ Get Started with Valu3 Today! ⚡**

## Examples :space_invader:

Here are some examples of how to use the Valu3:

## Serde converters (serde <-> `Value`)

Valu3 now exposes zero-copy converters between Serde `Serialize`/`Deserialize` types and `Value`, implemented using Serde's `Serializer`/`Deserializer` traits (no textual intermediate like `serde_json`). Use these when you need to convert Rust structs/enums/collections directly to `Value` and back.

Highlights
- Direct conversion without text: avoids serializing to JSON and reparsing.
- Full support for primitives, strings, arrays, maps/objects, datetimes, numbers and options.
- Enum support: unit variants, newtype variants, tuple variants and struct variants are supported. Representation conventions:
    - Unit variant: represented as a string with the variant name, e.g. `"MyVariant"`.
    - Newtype / Tuple / Struct variant: represented as a single-key object whose key is the variant name and whose value holds the inner payload, e.g. `{"Variant": value}` or `{"TupleVariant": [1,2]}` or `{"StructVariant": {"a":1}}`.

API
- `valu3::serde_value::to_value<T: Serialize>(&T) -> Result<Value, Error>` – serialize a `T` directly into a `Value`.
- `valu3::serde_value::from_value<T: DeserializeOwned>(&Value) -> Result<T, Error>` – deserialize a `Value` into `T`.

Example
```rust
use valu3::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum E {
        Unit,
        Newtype(String),
        Tuple(i32, i32),
        Struct { a: i32, b: String },
}

let v = E::Struct { a: 1, b: "x".to_string() };
let value = valu3::serde_value::to_value(&v).unwrap();
// value will be { "Struct": { "a": 1, "b": "x" } }

let back: E = valu3::serde_value::from_value(&value).unwrap();
assert_eq!(v, back);
```

Notes & limitations
- The converters prefer sensible numeric visitors (i64/u64/f64) when possible; extremely large integers are preserved using i128/u128 when necessary.
- Byte sequences are not supported by the serializer (they produce an error).
- Enum array-style representation (e.g. `["Variant", {...}]`) is not supported; file an issue if you need it.


```rust
use valu3::prelude::*;

let string_value = "hello".to_value();
let number_value = 42.to_value();
let boolean_value = true.to_value();
let null_value = Value::Null;
let undefined_value = Value::Undefined;
let mut datetime_value = DateTime::from("2023-04-05T00:00:00Z").to_value();

string_value.as_string();
number_value.get_i32();
assert_eq!(boolean_value, true);
assert_eq!(null_value, Value::Null);
assert_eq!(undefined_value, Value::Undefined);
datetime_value.add_days(1);
```

## Getting Started
To start using the Valu3 in your Rust project, simply add the following line to your `Cargo.toml` file:
```toml
[dependencies]
valu3 = "0.8"
```

Then, you can import the library in your code like this:
```rust
use valu3::prelude::*;

//...

let pi = 3.14.to_value();
```

## Supported Types 📦

Valu3 provides a comprehensive type system through the `Value` enum, which can represent eight fundamental data types:

### Overview of Types

1. **String** - Text data with extended manipulation methods
2. **Number** - All numeric types (integers and floats)
3. **Boolean** - True/false values
4. **Array** - Ordered collections of values
5. **Object** - Key-value mappings (HashMap or BTreeMap)
6. **DateTime** - Date, time, or datetime values
7. **Null** - Absence of value
8. **Undefined** - Uninitialized value

### 1. String Type (`StringB`)

The String type in Valu3 provides enhanced string manipulation capabilities.

**Key Methods:**
- `as_string()` - Get the String value
- `to_uppercase()` - Convert to uppercase
- `to_lowercase()` - Convert to lowercase
- `trim()` - Remove whitespace
- `replace()` - Replace substrings
- `concat()` - Concatenate strings
- `len()` - Get string length

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    // Creating strings
    let text = "Hello, World!".to_value();
    let mut name = Value::String(StringB::from("Alice"));
    
    // String operations
    if let Value::String(s) = &text {
        println!("Original: {}", s.as_string());
        println!("Uppercase: {}", s.to_uppercase().as_string());
        println!("Lowercase: {}", s.to_lowercase().as_string());
    }
    
    // More operations
    let trimmed = "  spaces  ".to_value();
    if let Value::String(s) = &trimmed {
        println!("Trimmed: '{}'", s.trim().as_string());
    }
    
    // Replace and concat
    let greeting = "Hello, NAME!".to_value();
    if let Value::String(s) = &greeting {
        let personalized = s.replace("NAME", "Bob");
        println!("Replaced: {}", personalized.as_string());
        
        let extended = personalized.concat(" How are you?");
        println!("Concatenated: {}", extended.as_string());
    }
}
```

### 2. Number Type

Supports all Rust numeric types with automatic type detection and conversion.

**Supported Types:**
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Floating point: `f32`, `f64`

**Key Methods:**
- `get_[type]()` - Get value as specific type (returns `Option`)
- `set_[type]()` - Set value as specific type
- `is_[type]()` - Check if value is specific type
- `is_integer()` - Check if integer
- `is_float()` - Check if float
- `is_positive()` / `is_negative()` - Check sign

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    // Creating numbers
    let int_value = 42.to_value();
    let float_value = 3.14159.to_value();
    let negative = (-100).to_value();
    
    // Type checking
    println!("42 is integer: {}", int_value.as_number().unwrap().is_integer());
    println!("3.14 is float: {}", float_value.as_number().unwrap().is_float());
    
    // Getting values
    if let Some(num) = int_value.as_number() {
        println!("As i32: {:?}", num.get_i32());
        println!("As f64: {:?}", num.get_f64());
    }
    
    // Setting values
    let mut dynamic_num = Value::Number(Number::default());
    dynamic_num.set_u64(1000000);
    println!("Set to u64: {:?}", dynamic_num.get_u64());
    
    // Sign checking
    if let Some(num) = negative.as_number() {
        println!("-100 is negative: {}", num.is_negative());
    }
}
```

### 3. Boolean Type

Simple true/false values.

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    let is_active = true.to_value();
    let is_complete = Value::Boolean(false);
    
    // Checking boolean values
    if let Value::Boolean(b) = is_active {
        println!("Active status: {}", b);
    }
    
    // Using in conditions
    match is_complete {
        Value::Boolean(true) => println!("Task completed!"),
        Value::Boolean(false) => println!("Task pending..."),
        _ => println!("Not a boolean"),
    }
}
```

### 4. Array Type

Ordered collection of `Value` items.

**Key Methods:**
- `push()` - Add element to end
- `pop()` - Remove and return last element
- `get()` - Get element by index
- `get_mut()` - Get mutable reference by index
- `len()` - Get array length
- `is_empty()` - Check if empty

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    // Creating arrays
    let mut numbers = vec![1, 2, 3, 4, 5].to_value();
    let mixed = json!(["text", 42, true, null]);
    
    // Array operations
    if let Value::Array(arr) = &mut numbers {
        arr.push(6.to_value());
        println!("Array length: {}", arr.len());
        
        // Accessing elements
        if let Some(first) = arr.get(0) {
            println!("First element: {}", first);
        }
        
        // Removing elements
        if let Some(last) = arr.pop() {
            println!("Popped: {}", last);
        }
    }
    
    // Iterating
    if let Value::Array(arr) = &mixed {
        for (index, value) in arr.values.iter().enumerate() {
            println!("Index {}: {}", index, value);
        }
    }
}
```

### 5. Object Type

Key-value mappings using either `HashMap` or `BTreeMap`.

**Key Methods:**
- `insert()` - Insert key-value pair
- `get()` - Get value by key
- `get_mut()` - Get mutable reference by key
- `remove()` - Remove key-value pair
- `contains_key()` - Check if key exists
- `keys()` - Get all keys
- `values()` - Get all values
- `len()` - Get number of pairs

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    // Creating objects
    let mut person = json!({
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com"
    });
    
    // Accessing values
    if let Some(name) = person.get("name") {
        println!("Name: {}", name);
    }
    
    // Modifying objects
    person.insert("city", "New York".to_value());
    person.insert("age", 31.to_value()); // Update existing
    
    // Checking keys
    if let Value::Object(obj) = &person {
        println!("Has email: {}", obj.contains_key(&"email"));
        println!("Object size: {}", obj.len());
        
        // Iterate over keys
        for key in obj.keys() {
            println!("Key: {}", key);
        }
    }
    
    // Remove values
    if let Value::Object(obj) = &mut person {
        if let Some(removed) = obj.remove(&"email") {
            println!("Removed email: {}", removed);
        }
    }
}
```

### 6. DateTime Type

Represents dates, times, or combined datetime values.

**Variants:**
- `Date(NaiveDate)` - Date without timezone
- `Time(NaiveTime)` - Time without date
- `DateTime(ChDateTime<Utc>)` - Full datetime with timezone

**Key Methods:**
- `year()`, `month()`, `day()` - Date components
- `hour()`, `minute()`, `second()` - Time components
- `to_iso8601()` - Format as ISO 8601
- `to_rfc3339()` - Format as RFC 3339
- `add_duration()` - Add time duration
- `now()` - Current datetime

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    // Creating datetime values
    let now = DateTime::now().to_value();
    let specific_date = DateTime::from("2024-01-15T10:30:00Z").to_value();
    let date_only = DateTime::from_ymd_opt(2024, 12, 25).to_value();
    
    // Accessing components
    if let Value::DateTime(dt) = &specific_date {
        println!("Year: {:?}", dt.year());
        println!("Month: {:?}", dt.month());
        println!("Hour: {:?}", dt.hour());
        println!("ISO 8601: {}", dt.to_iso8601());
    }
    
    // Date arithmetic
    if let Value::DateTime(dt) = &date_only {
        let tomorrow = dt.add_duration(chrono::Duration::days(1));
        if let Some(next_day) = tomorrow {
            println!("Tomorrow: {}", next_day);
        }
    }
}
```

### 7. Null and Undefined Types

Represent absence or uninitialized values.

**Example:**
```rust
use valu3::prelude::*;

fn main() {
    let null_value = Value::Null;
    let undefined_value = Value::Undefined;
    
    // Checking for null/undefined
    println!("Is null: {}", null_value.is_null());
    println!("Is undefined: {}", undefined_value.is_undefined());
    
    // Pattern matching
    match null_value {
        Value::Null => println!("Value is null"),
        Value::Undefined => println!("Value is undefined"),
        _ => println!("Value has content"),
    }
    
    // JSON representation
    println!("Null as JSON: {}", null_value.to_json(JsonMode::Compact));
    println!("Display: {}", undefined_value);
}
```

## Complete Examples 🎯

### Working with Complex Data Structures

```rust
use valu3::prelude::*;

fn main() {
    // Building a complex data structure
    let mut database = json!({
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "active": true,
                "roles": ["admin", "user"],
                "metadata": {
                    "created_at": "2024-01-01T00:00:00Z",
                    "last_login": null
                }
            },
            {
                "id": 2,
                "name": "Bob",
                "active": false,
                "roles": ["user"],
                "metadata": {
                    "created_at": "2024-01-15T00:00:00Z",
                    "last_login": "2024-01-20T10:30:00Z"
                }
            }
        ],
        "settings": {
            "max_users": 100,
            "allow_registration": true
        }
    });
    
    // Accessing nested data
    if let Some(users) = database.get("users") {
        if let Value::Array(user_array) = users {
            println!("Total users: {}", user_array.len());
            
            // Process each user
            for user in &user_array.values {
                if let Some(name) = user.get("name") {
                    println!("User: {}", name);
                }
            }
        }
    }
    
    // Modifying nested data
    if let Some(settings) = database.get_mut("settings") {
        settings.insert("maintenance_mode", false.to_value());
    }
}
```

### Type Conversions and Transformations

```rust
use valu3::prelude::*;

fn main() {
    // Converting between different representations
    let data = json!({
        "temperature": "23.5",
        "count": "42",
        "enabled": "true",
        "items": "[1,2,3]"
    });
    
    // Parse string values to appropriate types
    if let Some(temp_str) = data.get("temperature") {
        if let Value::String(s) = temp_str {
            let temp_float: f64 = s.as_string().parse().unwrap();
            println!("Temperature: {}°C", temp_float);
        }
    }
    
    // Convert JSON string to Value
    let json_str = r#"{"name":"Product","price":29.99}"#;
    let parsed = Value::payload_to_value(json_str).unwrap();
    println!("Parsed: {}", parsed.to_json(JsonMode::Compact));
    
    // Value to different formats
    let product = json!({
        "id": 123,
        "name": "Widget",
        "price": 19.99,
        "in_stock": true
    });
    
    // To JSON
    println!("JSON: {}", product.to_json(JsonMode::Indented));
    
    // To YAML (if feature enabled)
    // println!("YAML: {}", product.to_yaml());
}
```

### Data Validation and Processing

```rust
use valu3::prelude::*;

fn validate_user(user: &Value) -> Result<(), String> {
    // Check required fields
    if !user.is_object() {
        return Err("User must be an object".to_string());
    }
    
    // Validate email
    match user.get("email") {
        Some(email) if email.is_string() => {
            let email_str = email.as_string_b().unwrap().as_string();
            if !email_str.contains('@') {
                return Err("Invalid email format".to_string());
            }
        },
        _ => return Err("Email is required and must be a string".to_string()),
    }
    
    // Validate age
    if let Some(age) = user.get("age") {
        if let Some(num) = age.as_number() {
            if let Some(age_val) = num.get_i32() {
                if age_val < 0 || age_val > 150 {
                    return Err("Age must be between 0 and 150".to_string());
                }
            }
        }
    }
    
    Ok(())
}

fn main() {
    let valid_user = json!({
        "email": "user@example.com",
        "age": 25,
        "name": "John Doe"
    });
    
    let invalid_user = json!({
        "email": "not-an-email",
        "age": 200
    });
    
    match validate_user(&valid_user) {
        Ok(_) => println!("✓ Valid user"),
        Err(e) => println!("✗ Invalid user: {}", e),
    }
    
    match validate_user(&invalid_user) {
        Ok(_) => println!("✓ Valid user"),
        Err(e) => println!("✗ Invalid user: {}", e),
    }
}
```

### Dynamic Data Manipulation

```rust
use valu3::prelude::*;

fn merge_objects(base: &mut Value, updates: &Value) {
    if let (Value::Object(base_obj), Value::Object(updates_obj)) = (base, updates) {
        for (key, value) in updates_obj.keys().zip(updates_obj.values()) {
            base_obj.insert(key.clone(), value.clone());
        }
    }
}

fn filter_array(array: &Value, predicate: impl Fn(&Value) -> bool) -> Value {
    if let Value::Array(arr) = array {
        let filtered: Vec<Value> = arr.values.iter()
            .filter(|v| predicate(v))
            .cloned()
            .collect();
        return filtered.to_value();
    }
    Value::Array(Array::new())
}

fn main() {
    // Merging objects
    let mut config = json!({
        "host": "localhost",
        "port": 8080
    });
    
    let overrides = json!({
        "port": 3000,
        "debug": true
    });
    
    merge_objects(&mut config, &overrides);
    println!("Merged config: {}", config);
    
    // Filtering arrays
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_value();
    let evens = filter_array(&numbers, |v| {
        if let Some(num) = v.as_number() {
            if let Some(n) = num.get_i32() {
                return n % 2 == 0;
            }
        }
        false
    });
    
    println!("Even numbers: {}", evens);
}
```

### Working with the json! Macro

```rust
use valu3::prelude::*;

fn main() {
    // The json! macro supports all JSON syntax
    let config = json!({
        "application": {
            "name": "MyApp",
            "version": "1.0.0",
            "features": [
                "auth",
                "api",
                "dashboard"
            ],
            "settings": {
                "theme": "dark",
                "language": "en",
                "notifications": true
            }
        },
        "database": {
            "host": "localhost",
            "port": 5432,
            "ssl": false,
            "pools": {
                "min": 5,
                "max": 20
            }
        },
        "cache": null,
        "debug": true
    });
    
    // Variables in json! macro
    let app_name = "MyService";
    let version = 2;
    let is_production = false;
    
    let service = json!({
        "service": app_name,
        "version": version,
        "production": is_production,
        "endpoints": [
            "/api/v1",
            "/api/v2"
        ]
    });
    
    println!("Service config: {}", service.to_json(JsonMode::Compact));
}
```

## Structs and Conversions
Valu3 natively has conversions for famous data types like json, yaml and xml. Furthermore with `valu3-derive` you are able to transform `struct` to `Value` by applying the `to_value()` method generated by the `ToValue` derive macros. This is an example on converting `struct` to `Value` and `Value` to other payload data types.

```rust
use valu3::prelude:*;

#[derive(ToValue, FromValue, Default)]
struct MyStruct {
    id: u32,
    name: String,
    tags: Vec<String>
}

fn main(){
    let my_struct = MyStruct::default();
    let value = my_struct.to_value();

    assert_eq!(my_struct, MyStruct::from_value(value));
}

```

### ToJson 
If your focus is only on using `Valu3` for conversion only, use the `ToJson` macro.

```rust
use valu3::prelude:*;

#[derive(ToJson, Default)]
struct MyStruct {
    id: u32,
    name: String,
    tags: Vec<String>
}

fn main(){
    let my_struct = MyStruct::default();
    let json = my_struct.to_json();

    println!("{}", json); // print json string
}

```

## Payload

`Vale3` is able to recognize a payload string, identify and convert it to `Value`, follow the example:

```rust
use valu3::prelude:*;
fn main(){
    let boolean = Value::payload_to_value("true").unwrap();
    let float = Value::payload_to_value("3.14").unwrap();
    let json = Value::payload_to_value(r#"{"item": 3.14}"#).unwrap();
    let array = Value::payload_to_value(r#"[1,2,3]"#).unwrap();
    let null = Value::payload_to_value("null").unwrap();
    let string = Value::payload_to_value(r#""123""#).unwrap();

    assert_eq!(boolean, true.to_value());
    assert_eq!(float, 3.14.to_value());
    assert_eq!(json, Value::from(vec![("item", 3.14)]));
    assert_eq!(array, vec![1, 2, 3].to_value());
    assert_eq!(null, Value::Null);
    assert_eq!(string, "123".to_value());
}

```

## Contributing
If you find a bug or have a suggestion for a new feature, please open an issue on the [GitHub repository](https://github.com/lowcarboncode/valu3/issues).

If you would like to contribute to the project, please feel free to submit a pull request. Before submitting a pull request, please make sure that your code adheres to the project's style guidelines and passes all tests.

## Upcoming Features: Stay in Sync with the Future of Valu3 🌐
~
We're constantly working to improve and expand the capabilities of Valu3, making it even more powerful and versatile. 

By keeping track of the project's progress, you can stay informed about new features in development and planned improvements. This will allow you to make the most of Valu3 in your Rust projects and prepare for future updates.

Our commitment is to make Valu3 the ultimate data manipulation solution in Rust. Your input is invaluable! Feel free to join the discussions, share your ideas, and contribute to the project as it evolves.

Join us in the ongoing journey to refine and expand Valu3! 🚀


## License
This project is licensed under the Apache 2.0 License. See the [LICENSE](https://github.com/lowcarboncode/valu3/blob/main/LICENSE) file for more information.
