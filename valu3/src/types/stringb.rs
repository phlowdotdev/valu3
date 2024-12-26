//! A custom string implementation called `StringB` that provides additional string manipulation methods.
//!
//! This implementation offers a way to handle strings with additional features, such as converting
//! the string to uppercase or lowercase, trimming, replacing, and concatenating. It also handles
//! converting between different representations of strings, such as `CString`, `String`, and `Vec<u8>`.
#[cfg(feature = "cstring")]
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub trait StringBehavior {
    /// Gets the byte representation of the string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("hello");
    /// let bytes = s.as_bytes();
    /// ```
    fn as_bytes(&self) -> &[u8];

    /// Gets the string slice representation of the value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("hello");
    /// let slice = s.as_str();
    /// ```
    fn as_str(&self) -> &str;

    #[cfg(feature = "cstring")]
    fn extract(&self) -> CString;

    #[cfg(not(feature = "cstring"))]
    fn extract(&self) -> String;

    /// Converts the value to a `String`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("hello");
    /// let string = s.as_string();
    /// ```
    fn as_string(&self) -> String;
    /// Converts the string to uppercase.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("hello");
    /// assert_eq!(s.to_uppercase().as_str(), "HELLO");
    /// ```
    fn to_uppercase(&self) -> Self;

    /// Converts the string to lowercase.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("HELLO");
    /// assert_eq!(s.to_lowercase().as_str(), "hello");
    /// ```
    fn to_lowercase(&self) -> Self;

    /// Removes whitespace at the beginning and end of the string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("  hello  ");
    /// assert_eq!(s.trim().as_str(), "hello");
    /// ```
    fn trim(&self) -> Self;

    /// Replaces all occurrences of 'from' with 'to'.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::from("hello world");
    /// assert_eq!(s.replace("world", "planet").as_str(), "hello planet");
    /// ```
    fn replace(&self, from: &str, to: &str) -> Self;

    /// Concatenates the current string with another string or `&str`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s1 = StringB::from("hello");
    /// let s2 = " world";
    /// assert_eq!(s1.concat(s2).as_str(), "hello world");
    /// ```
    fn concat<T: AsRef<str>>(&self, other: T) -> Self;

    fn from_utf8(value: Vec<u8>) -> Self;
}

/// A custom string implementation with additional manipulation methods.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct StringB {
    #[cfg(feature = "cstring")]
    pub value: CString,
    #[cfg(not(feature = "cstring"))]
    pub value: String,
}

impl StringB {
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(feature = "cstring")]
    pub fn as_string(&self) -> String {
        self.value.to_str().unwrap().to_string()
    }

    #[cfg(not(feature = "cstring"))]
    pub fn as_string(&self) -> String {
        self.value.clone()
    }
}

impl StringBehavior for StringB {
    fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    #[cfg(feature = "cstring")]
    fn as_str(&self) -> &str {
        self.value.to_str().expect("CString is not valid UTF-8")
    }

    #[cfg(not(feature = "cstring"))]
    fn as_str(&self) -> &str {
        self.value.as_str()
    }

    #[cfg(feature = "cstring")]
    fn as_string(&self) -> String {
        self.value
            .to_str()
            .expect("CString is not valid UTF-8")
            .to_string()
    }

    #[cfg(not(feature = "cstring"))]
    fn as_string(&self) -> String {
        self.value.clone()
    }

    #[cfg(feature = "cstring")]
    fn extract(&self) -> CString {
        self.value.clone()
    }

    #[cfg(not(feature = "cstring"))]
    fn extract(&self) -> String {
        self.as_string()
    }

    fn to_uppercase(&self) -> Self {
        let upper_str = self.as_str().to_uppercase();
        StringB::from(upper_str)
    }

    fn to_lowercase(&self) -> Self {
        let lower_str = self.as_str().to_lowercase();
        StringB::from(lower_str)
    }

    fn trim(&self) -> Self {
        let trimmed_str = self.as_str().trim();
        StringB::from(trimmed_str)
    }

    fn replace(&self, from: &str, to: &str) -> Self {
        let replaced_str = self.as_str().replace(from, to);
        StringB::from(replaced_str)
    }

    fn concat<T: AsRef<str>>(&self, other: T) -> Self {
        let mut result = String::from(self.as_str());
        result.push_str(other.as_ref());
        StringB::from(result)
    }

    #[cfg(not(feature = "cstring"))]
    fn from_utf8(value: Vec<u8>) -> Self {
        StringB::from(String::from_utf8(value).unwrap())
    }

    #[cfg(feature = "cstring")]
    fn from_utf8(value: Vec<u8>) -> Self {
        StringB {
            value: CString::new(value).unwrap(),
        }
    }
}

/// Implements the `Display` trait for `StringB`.
///
/// This allows `StringB` instances to be formatted using the `{}` placeholder in format strings.
#[cfg(feature = "cstring")]
impl Display for StringB {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
#[cfg(not(feature = "cstring"))]
impl Display for StringB {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// Implements the `Deref` trait for `StringB`.
///
/// This allows treating a `StringB` instance as if it were a slice of bytes (`[u8]`).
impl Deref for StringB {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

/// Implements the `From<String>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `String`.
#[cfg(not(feature = "cstring"))]
impl From<String> for StringB {
    fn from(value: String) -> Self {
        StringB {
            value: value.clone(),
        }
    }
}

/// Implements the `From<&str>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `&str`.
#[cfg(not(feature = "cstring"))]
impl From<&str> for StringB {
    fn from(value: &str) -> Self {
        StringB {
            value: value.to_string(),
        }
    }
}

#[cfg(feature = "cstring")]
impl From<String> for StringB {
    fn from(value: String) -> Self {
        StringB {
            value: CString::new(value).unwrap(),
        }
    }
}

#[cfg(feature = "cstring")]
impl From<&str> for StringB {
    fn from(value: &str) -> Self {
        StringB {
            value: CString::new(value).unwrap(),
        }
    }
}

#[cfg(feature = "cstring")]
impl From<CString> for StringB {
    fn from(value: CString) -> Self {
        StringB {
            value: value.clone(),
        }
    }
}

/// Implements the `From<&Vec<u8>>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a reference to a `Vec<u8>`.
impl From<&Vec<u8>> for StringB {
    fn from(value: &Vec<u8>) -> Self {
        StringB::from_utf8(value.clone())
    }
}

/// Implements the `From<Vec<u8>>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `Vec<u8>`.
impl From<Vec<u8>> for StringB {
    fn from(value: Vec<u8>) -> Self {
        StringB::from_utf8(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let s = StringB::from("Hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_is_empty() {
        let s = StringB::from("");
        assert!(s.is_empty());
    }

    #[test]
    fn test_to_uppercase() {
        let s = StringB::from("hello");
        assert_eq!(s.to_uppercase().as_str(), "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        let s = StringB::from("HELLO");
        assert_eq!(s.to_lowercase().as_str(), "hello");
    }

    #[test]
    fn test_trim() {
        let s = StringB::from("  hello  ");
        assert_eq!(s.trim().as_str(), "hello");
    }

    #[test]
    fn test_replace() {
        let s = StringB::from("hello world");
        assert_eq!(s.replace("world", "planet").as_str(), "hello planet");
    }

    #[test]
    fn test_concat() {
        let s1 = StringB::from("hello");
        let s2 = " world";
        assert_eq!(s1.concat(s2).as_str(), "hello world");
    }
}
