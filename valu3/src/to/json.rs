use crate::prelude::*;
use regex::Regex;

/// An enum representing the JSON output format mode.
pub enum JsonMode {
    /// Outputs the JSON in an indented format.
    Indented,
    /// Outputs the JSON in an inline format.
    Inline,
}

impl Value {
    /// Converts a `Value` into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `mode` - A `JsonMode` value representing the JSON output format mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use json_utils::{Value, JsonMode};
    ///
    /// let value = Value::payload_to_value("{\"name\":\"John Doe\",\"age\":30,\"is_active\":true}").unwrap();
    /// let json_string = value.to_json(JsonMode::Indented);
    /// println!("{}", json_string);
    /// ```
    pub fn to_json(&self, mode: JsonMode) -> String {
        let value = Value::to_json_inner(self, 0);

        match mode {
            JsonMode::Inline => Self::inline(value),
            JsonMode::Indented => value,
        }
    }

    /// Converts the inline JSON string into an indented JSON string.
    fn inline(value: String) -> String {
        let re = Regex::new(r"(\n)|(\t)").unwrap();
        let result = re.replace_all(&value, "");
        result.to_string()
    }

    /// Generates tab indentation.
    fn tabs(total: i32) -> String {
        vec!["\t"; total as usize].join("")
    }

    /// Converts a `Value` into a JSON string.
    fn to_json_inner(val: &Value, children: i32) -> String {
        match val {
            Value::Object(o) => {
                let contents: Vec<_> = o
                    .iter()
                    .map(|(name, value)| {
                        format!(
                            "\n\t{}\"{}\": {}",
                            &Self::tabs(children),
                            name,
                            Value::to_json_inner(value, children + 1)
                        )
                    })
                    .collect();
                format!("{{{}\n{}}}", contents.join(","), &Self::tabs(children))
            }
            Value::Array(a) => {
                let contents: Vec<_> = a
                    .into_iter()
                    .map(|value| Value::to_json_inner(value, children + 1))
                    .collect();
                format!(
                    "[\n\t{}{}\n{}]",
                    &Self::tabs(children),
                    contents.join(&format!(",\n\t{}", &Self::tabs(children))),
                    &Self::tabs(children)
                )
            }
            Value::String(s) => {
                let mut json = serde_json::to_string(s.as_str()).unwrap();
                // Normalização geral: remove espaços entre uma aspa escapada e '>'
                // Ex.: transforma \"   > em \">
                let re = Regex::new(r#"(\\")\s+>"#).unwrap();
                json = re.replace_all(&json, "$1>").to_string();
                json
            }
            Value::Number(n) => format!("{}", n),
            Value::Boolean(b) => format!("{}", b),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::DateTime(date_time) => format!("\"{}\"", date_time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_remove_tabs_and_empty_lines() {
        let str =
            String::from("{\n\t\"name\":\"John Doe\",\n\t\"age\":30,\n\t\"is_active\":true\n}");
        let expected = String::from("{\"name\":\"John Doe\",\"age\":30,\"is_active\":true}");
        assert_eq!(expected, Value::inline(str));
    }

    #[test]
    fn it_should_add_tabs_by_number() {
        assert_eq!("\t\t\t", Value::tabs(3));
    }

    #[test]
    fn it_should_convert_a_value_to_json_string() {
        let value_str = Value::json_to_value("{\"name\":\"John Doe\"}").unwrap();
        let value_number = Value::json_to_value("{\"age\":30}").unwrap();
        let value_boolean = Value::json_to_value("{\"is_active\":true}").unwrap();
        assert_eq!(
            "{\n\t\"name\": \"John Doe\"\n}",
            value_str.to_json(JsonMode::Indented)
        );
        assert_eq!(
            "{\n\t\"age\": 30\n}",
            value_number.to_json(JsonMode::Indented)
        );
        assert_eq!(
            "{\n\t\"is_active\": true\n}",
            value_boolean.to_json(JsonMode::Indented)
        )
    }

    #[test]
    fn it_should_complex_string() {
        let string = r#"<img src="image.jpg" alt="An image" >"#;
        let map = vec![("html".to_string(), Value::from(string))];
        let value = Value::from(map);
        let json_output = value.to_json(JsonMode::Indented);

        let expected = "{\n\t\"html\": \"<img src=\\\"image.jpg\\\" alt=\\\"An image\\\">\"\n}";
        assert_eq!(expected, json_output);
    }
}
