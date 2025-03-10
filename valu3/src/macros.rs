#[macro_export]
macro_rules! json {
    (null) => {
        $crate::value::Value::Null
    };
    (true) => {
        $crate::value::Value::from(true)
    };
    (false) => {
        $crate::value::Value::from(false)
    };
    ([]) => {
        $crate::value::Value::from(vec![])
    };
    ([ $($elem:tt),* ]) => {
        $crate::value::Value::from(vec![$(json!($elem)),*])
    };
    ({}) => {
        $crate::value::Value::from(std::collections::HashMap::new())
    };
    ({ $($key:tt : $value:tt),* }) => {
        $crate::value::Value::from({
            let mut map = std::collections::HashMap::new();
            $(map.insert($key.to_string(), json!($value));)*
            map
        })
    };
    ($val:expr) => {
        $val.to_value()
    };
}

#[cfg(test)]
mod test {
    use crate::traits::ToValueBehavior;
    use std::collections::HashMap;

    #[test]
    fn test_json() {
        let json = json!({
            "test": true,
            "test2": "ok",
            "test3": [0, 1],
            "test4": {
                "test5": true,
                "test6": "ok",
                "test7": [0, 1]
            }
        });

        let mut map = HashMap::new();
        map.insert("test".to_string(), true.to_value());
        map.insert("test2".to_string(), "ok".to_string().to_value());
        map.insert("test3".to_string(), vec![0, 1].to_value());

        let mut inner_map = HashMap::new();
        inner_map.insert("test5".to_string(), true.to_value());
        inner_map.insert("test6".to_string(), "ok".to_value());
        inner_map.insert("test7".to_string(), vec![0, 1].to_value());

        map.insert("test4".to_string(), inner_map.to_value());

        assert_eq!(json, map.to_value());
    }
}
