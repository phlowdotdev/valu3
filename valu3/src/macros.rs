// Simple json! macro for basic JSON-like syntax
// Note: This is a simplified version and doesn't support all JSON features
#[macro_export]
macro_rules! json {
    // Handle null
    (null) => {
        $crate::value::Value::Null
    };
    
    // Handle booleans
    (true) => {
        $crate::value::Value::from(true)
    };
    (false) => {
        $crate::value::Value::from(false)
    };
    
    // Handle any other expression
    ($val:expr) => {{
        use $crate::traits::ToValueBehavior;
        ($val).to_value()
    }};
}

#[cfg(test)]
mod test {
    use crate::traits::ToValueBehavior;
    use std::collections::HashMap;

    #[test]
    fn test_json() {
        // Test basic values
        let null = json!(null);
        assert_eq!(null, crate::value::Value::Null);
        
        let bool_true = json!(true);
        assert_eq!(bool_true, crate::value::Value::from(true));
        
        let bool_false = json!(false);
        assert_eq!(bool_false, crate::value::Value::from(false));
        
        // Test expressions
        let number = json!(42);
        assert_eq!(number, 42.to_value());
        
        let string = json!("hello");
        assert_eq!(string, "hello".to_value());
        
        let array = json!(vec![1, 2, 3]);
        assert_eq!(array, vec![1, 2, 3].to_value());
        
        // Test with variables
        let test_val = true;
        let from_var = json!(test_val);
        assert_eq!(from_var, test_val.to_value());
        
        // Test complex structure built with Rust syntax
        let mut map = HashMap::new();
        map.insert("test".to_string(), true.to_value());
        map.insert("test2".to_string(), "ok".to_value());
        map.insert("test3".to_string(), vec![0, 1].to_value());
        
        let complex = json!(map);
        assert_eq!(complex, map.to_value());
    }
}
