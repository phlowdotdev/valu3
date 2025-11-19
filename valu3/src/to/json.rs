use crate::prelude::*;
use regex::Regex;
use serde_json::{self, Map, Value as SerdeValue};

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

    pub fn to_json(&self, mode: JsonMode) -> String {
        match self.to_serde_json_value() {
            Ok(serde_value) => match mode {
                JsonMode::Inline => serde_value,
                JsonMode::Indented => Self::idented(serde_value),
            },
            Err(e) => format!("Error converting to JSON: {}", e),
        }
    }

    /// Converte o Value interno em serde_json::Value usando apenas APIs de serde.
    fn to_serde_json_value(&self) -> std::result::Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    /// Manual identação de strings JSON
    fn idented(value: String) -> String {
        let v: Result<SerdeValue, _> = serde_json::from_str(&value);
        match v {
            Ok(json_value) => serde_json::to_string_pretty(&json_value).unwrap_or(value),
            Err(_) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_a_value_to_json_string() {
        let value_str = Value::json_to_value("{\"name\":\"John Doe\"}").unwrap();
        let value_number = Value::json_to_value("{\"age\":30}").unwrap();
        let value_boolean = Value::json_to_value("{\"is_active\":true}").unwrap();

        assert_eq!(
            "{\n  \"name\": \"John Doe\"\n}",
            value_str.to_json(JsonMode::Indented)
        );
        assert_eq!(
            "{\n  \"age\": 30\n}",
            value_number.to_json(JsonMode::Indented)
        );
        assert_eq!(
            "{\n  \"is_active\": true\n}",
            value_boolean.to_json(JsonMode::Indented)
        );
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
