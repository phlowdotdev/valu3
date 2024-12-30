use crate::prelude::*;
use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/json/json.pest"]
struct JSONParser;

use pest::iterators::Pair;

impl Value {
    pub fn json_to_value(str: &str) -> Result<Value, Error> {
        let value = match JSONParser::parse(Rule::json, str.trim()) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => Self::json_parse_value_inner(pair),
                None => return Err(Error::NonParseble),
            },
            Err(msg) => return Err(Error::NonParsebleMsg(msg.to_string())),
        };
        Ok(value)
    }

    fn json_parse_value_inner(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::object => {
                let map = pair
                    .into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .to_string();
                        let value = Self::json_parse_value_inner(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect::<HashMap<String, Value>>();

                Self::from(map)
            }
            Rule::array => Self::from(
                pair.into_inner()
                    .map(Self::json_parse_value_inner)
                    .collect::<Vec<_>>(),
            ),
            Rule::string => Self::from(StringB::from(pair.into_inner().next().unwrap().as_str())),
            Rule::number => Self::from(Number::try_from(pair.as_str()).unwrap()),
            Rule::boolean => Self::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => Self::Null,
            Rule::json
            | Rule::EOI
            | Rule::key_value_pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => Self::Undefined,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn json() {
        let raw: &str = "{
        \"test\": true,
        \"test2\": \"ok\",
        \"test3\": [0, 1]
       }";

        let compare = Value::from({
            let mut map = HashMap::new();
            map.insert("test".to_string(), true.to_value());
            map.insert("test2".to_string(), "ok".to_value());
            map.insert("test3".to_string(), Value::from(vec![0, 1]));
            map
        });

        assert_eq!(Value::json_to_value(raw), Ok(compare));
    }

    #[test]
    fn array() {
        let raw = "[0, true, null, \"ok\"]";

        let compare = {
            let mut list = Vec::new();
            list.push(Value::Number(Number::from(0)));
            list.push(Value::Boolean(true));
            list.push(Value::Null);
            list.push(Value::String(StringB::from("ok")));
            Value::from(list)
        };

        assert_eq!(Value::json_to_value(raw), Ok(compare));
    }

    #[test]
    fn number() {
        let int = "0";
        let float = "1.0";

        assert_eq!(
            Value::json_to_value(int),
            Ok(Value::Number(Number::from(0)))
        );
        assert_eq!(
            Value::json_to_value(float),
            Ok(Value::Number(Number::from(1.0)))
        );
    }

    #[test]
    fn string() {
        let string = r#""string""#;

        assert_eq!(
            Value::json_to_value(string),
            Ok(Value::String(StringB::from("string")))
        );
    }

    #[test]
    fn null() {
        let null = "null";

        assert_eq!(Value::json_to_value(null), Ok(Value::Null));
    }
    #[test]
    fn boolean() {
        let boolean = "true";

        assert_eq!(Value::json_to_value(boolean), Ok(Value::Boolean(true)));
    }

    #[test]
    fn all() {
        let boolean = Value::json_to_value("true").unwrap();
        let float = Value::json_to_value("3.14").unwrap();
        let json = Value::json_to_value(r#"{"item": 3.14}"#).unwrap();
        let array = Value::json_to_value(r#"[1,2,3]"#).unwrap();
        let null = Value::json_to_value("null").unwrap();
        let string = Value::json_to_value(r#""123""#).unwrap();

        assert_eq!(boolean, true.to_value());
        assert_eq!(float, 3.14.to_value());
        assert_eq!(json, Value::from(vec![("item", 3.14)]));
        assert_eq!(array, vec![1, 2, 3].to_value());
        assert_eq!(null, Value::Null);
        assert_eq!(string, "123".to_value());
    }
}
