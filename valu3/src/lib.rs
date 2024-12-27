//! # value
//! The Value library provides a generic value type with a wide range of classic methods for numerical and string manipulation. The library includes types for arrays, objects, numbers, strings, and datetime. It also provides methods for converting values to and from various data formats, such as JSON, YAML, and XML.
//!
//! With the Value library, you can easily manipulate different types of data in your Rust projects. You can create, modify, and query objects and arrays, perform arithmetic and bitwise operations on numbers, and convert values to and from strings. The library is easy to use and provides a consistent API for manipulating values, regardless of their type.
//!
//! Whether you're working on a small Rust project or a large-scale application, the Value library can help simplify your code and make it more manageable. Its simple and intuitive API makes it easy to work with, even for beginners.
//!
//! # Examples
//!
//! ```
//! use valu3::prelude::*;
//!
//! let string_value = hello".to_value();
//! let number_value = 42.to_value();
//! let boolean_value = true.to_value();
//! let null_value = Value::Null;
//! let undefined_value = Value::Undefined;
//! let mut datetime_value = DateTime::from("2023-04-05T00:00:00Z").to_value();
//!
//! string_value.as_string();
//! number_value.get_i32();
//! assert!(boolean_value, true);
//! assert!(null_value, Value::Null);
//! assert!(undefined_value, Value::Undefined);
//! datetime_value.add_days(1);
//! ```
pub mod impls;
pub mod prelude;
pub mod primitives;
#[cfg(feature = "serde")]
pub mod serde_value;
pub mod to;
pub mod to_value;
pub mod traits;
pub mod types;
pub mod value;

#[cfg(feature = "parser")]
#[macro_use]
extern crate pest_derive;

#[cfg(feature = "parser")]
pub mod parser;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    #[cfg(feature = "parser")]
    NonParsebleMsg(String),
    #[cfg(feature = "parser")]
    NonParseble,
    NotNumber,
}

#[cfg(test)]
mod tests;
