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
    pub fn to_json_idented(&self) -> String {
        self.to_json(JsonMode::Indented)
    }

    pub fn to_json_inline(&self) -> String {
        self.to_json(JsonMode::Inline)
    }

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
                let string = s.as_str();
                let mut out = String::with_capacity(string.len());
                let mut prev: Option<char> = None;
                for ch in string.chars() {
                    if ch == '"' && prev != Some('\\') {
                        out.push('\\');
                        out.push('"');
                    } else {
                        out.push(ch);
                    }
                    prev = Some(ch);
                }
                format!("\"{}\"", out)
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
        let string = r#"1 1763496849266 https://mercado.carrefour.com.br/mapa-do-site/1 <!DOCTYPE html><html lang="pt-BR"><head><link href="https://cdn-prod.securiti.ai/consent/cookie-consent-latest.css" rel="stylesheet"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0,user-scalable=0"><title>Mapa do Site | Supermercado Carrefour</title><meta name="robots" content="index,follow"><meta name="description" content="O Carrefour também tem supermercado online! Faça sua lista de compras e aproveite nosso delivery. Aproveite!"><meta property="og:title" content="Mercado Carrefour: Ofertas de Supermercado Delivery"><meta property="og:description" content="O Carrefour também tem supermercado online! Faça sua lista de compras e aproveite nosso delivery. Aproveite!"><meta property="og:url" content="https://mercado.carrefour.com.br"><meta property="og:type" content="website"><link rel="canonical" href="https://mercado.carrefour.com.br"><meta name="next-head-count" content="10"><meta name="google-site-verification" content="GjAwJWf5U8gd7i0Tg-Dqz8LE0qi4RWdMWxfwsd-EgOY"><meta name="facebook-domain-verification" content="ym08vcfms00jx3fkqdkgqgsxrxbi8f"><meta name="facebook-domain-verification" content="ym08vcfms00jx3fkqdkgqgsxrxbi8f"><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preload" href="https://fonts.googleapis.com"><link href="https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&amp;display=swap" rel="preload"><link href="https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,700;1,300&amp;display=swap" rel="preload"><link rel="preload" href="/_next/static/css/4a6cfdceadc6be2d.css" as="style"><link rel="stylesheet" href="/_next/static/css/4a6cfdceadc6be2d.css" data-n-g=""><link rel="preload" href="/_next/static/css/d2bb7ebb3aa1fe96.css" as="style"><link rel="stylesheet" href="/_next/static/css/d2bb7ebb3aa1fe96.css" data-n-p=""><noscript data-n-css=""></noscript><script defer="" nomodule="" src="/_next/static/chunks/polyfills-c67a75d1b6f99dc8.js"></script><script data-partytown-config="">"#;
        let value = Value::from(string);
        let json_output = value.to_json(JsonMode::Indented);
        // Esperado: objeto JSON com a chave "html" e a string com aspas internas escapadas
        let expected = r#""1 1763496849266 https://mercado.carrefour.com.br/mapa-do-site/1 <!DOCTYPE html><html lang=\"pt-BR\"><head><link href=\"https://cdn-prod.securiti.ai/consent/cookie-consent-latest.css\" rel=\"stylesheet\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, maximum-scale=1.0,user-scalable=0\"><title>Mapa do Site | Supermercado Carrefour</title><meta name=\"robots\" content=\"index,follow\"><meta name=\"description\" content=\"O Carrefour também tem supermercado online! Faça sua lista de compras e aproveite nosso delivery. Aproveite!\"><meta property=\"og:title\" content=\"Mercado Carrefour: Ofertas de Supermercado Delivery\"><meta property=\"og:description\" content=\"O Carrefour também tem supermercado online! Faça sua lista de compras e aproveite nosso delivery. Aproveite!\"><meta property=\"og:url\" content=\"https://mercado.carrefour.com.br\"><meta property=\"og:type\" content=\"website\"><link rel=\"canonical\" href=\"https://mercado.carrefour.com.br\"><meta name=\"next-head-count\" content=\"10\"><meta name=\"google-site-verification\" content=\"GjAwJWf5U8gd7i0Tg-Dqz8LE0qi4RWdMWxfwsd-EgOY\"><meta name=\"facebook-domain-verification\" content=\"ym08vcfms00jx3fkqdkgqgsxrxbi8f\"><meta name=\"facebook-domain-verification\" content=\"ym08vcfms00jx3fkqdkgqgsxrxbi8f\"><link rel=\"preconnect\" href=\"https://fonts.googleapis.com\"><link rel=\"preload\" href=\"https://fonts.googleapis.com\"><link href=\"https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&amp;display=swap\" rel=\"preload\"><link href=\"https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,700;1,300&amp;display=swap\" rel=\"preload\"><link rel=\"preload\" href=\"/_next/static/css/4a6cfdceadc6be2d.css\" as=\"style\"><link rel=\"stylesheet\" href=\"/_next/static/css/4a6cfdceadc6be2d.css\" data-n-g=\"\"><link rel=\"preload\" href=\"/_next/static/css/d2bb7ebb3aa1fe96.css\" as=\"style\"><link rel=\"stylesheet\" href=\"/_next/static/css/d2bb7ebb3aa1fe96.css\" data-n-p=\"\"><noscript data-n-css=\"\"></noscript><script defer=\"\" nomodule=\"\" src=\"/_next/static/chunks/polyfills-c67a75d1b6f99dc8.js\"></script><script data-partytown-config=\"\">""#;

        assert_eq!(json_output, expected);
    }
}
