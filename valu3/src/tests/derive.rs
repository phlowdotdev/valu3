#[cfg(test)]
mod test {
    use crate::prelude::*;
    use std::collections::{BTreeMap, HashMap};

    #[derive(ToValue, FromValue, PartialEq, Debug, Clone)]
    enum ExampleType {
        Example1,
        Example2,
    }

    #[derive(ToValue, FromValue, PartialEq, Debug, Default, Clone, ToJson)]
    struct Inner {
        item_a: bool,
        tree: BTreeMap<String, String>,
    }

    #[derive(ToValue, FromValue, PartialEq, Debug, Clone, ToJson)]
    struct Example<T>
    where
        T: PrimitiveType + FromValueBehavior<Item = T> + Clone + ToValueBehavior,
    {
        item_a: i32,
        item_b: String,
        item_c: Option<Vec<String>>,
        item_d: HashMap<String, Inner>,
        item_e: ExampleType,
        item_f: T,
    }

    #[test]
    fn test_example() {
        let example = Example {
            item_a: 1,
            item_b: "Hello".to_string(),
            item_c: Some(vec!["World".to_string()]),
            item_d: HashMap::default(),
            item_e: ExampleType::Example1,
            item_f: "Generic".to_string(),
        };

        let value = example.to_value();

        assert_eq!(example, Example::from_value(value).unwrap());
    }
}
