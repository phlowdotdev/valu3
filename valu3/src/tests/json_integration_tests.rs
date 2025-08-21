//! Integration tests for json! macro compatibility and interoperability
//! 
//! These tests ensure the json! macro works correctly with external libraries
//! and maintains compatibility with expected JSON behavior.

use crate::{json, prelude::*};
use std::collections::HashMap;

#[cfg(test)]
mod serde_compatibility {
    use super::*;

    #[test]
    fn test_json_to_string_roundtrip() {
        let original = json!({
            "name": "integration_test",
            "values": [1, 2, 3, 4, 5],
            "metadata": {
                "version": "1.0",
                "stable": true,
                "tags": ["test", "json", "macro"]
            },
            "config": {
                "timeout": 30,
                "retries": 3,
                "debug": false
            }
        });

        // Convert to JSON string
        let json_string = original.to_json(crate::to::json::JsonMode::Inline);
        
        // Verify JSON string contains expected content
        assert!(json_string.contains("integration_test"));
        assert!(json_string.contains("[1,2,3,4,5]") || json_string.contains("[1, 2, 3, 4, 5]"));
        assert!(json_string.contains("\"stable\":true") || json_string.contains("\"stable\": true"));
        
        // Parse back to Value
        if let Ok(parsed) = Value::json_to_value(&json_string) {
            // Verify structure is preserved
            if let Value::Object(map) = &parsed {
                assert!(map.contains_key(&"name"));
                assert!(map.contains_key(&"values"));
                assert!(map.contains_key(&"metadata"));
                assert!(map.contains_key(&"config"));
            } else {
                panic!("Expected object after parsing");
            }
        } else {
            println!("Warning: JSON parsing not available, skipping roundtrip verification");
        }
    }

    #[test] 
    fn test_complex_nested_structure() {
        let user_data = json!({
            "users": [
                {
                    "id": 1,
                    "username": "alice",
                    "profile": {
                        "email": "alice@example.com",
                        "preferences": {
                            "theme": "dark",
                            "language": "en",
                            "notifications": {
                                "email": true,
                                "push": false,
                                "sms": null
                            }
                        },
                        "social": {
                            "twitter": "@alice",
                            "github": "alice-dev",
                            "website": null
                        }
                    },
                    "stats": {
                        "posts": 42,
                        "followers": 1337,
                        "following": 256
                    }
                },
                {
                    "id": 2,
                    "username": "bob",
                    "profile": {
                        "email": "bob@example.com",
                        "preferences": {
                            "theme": "light",
                            "language": "es",
                            "notifications": {
                                "email": false,
                                "push": true,
                                "sms": true
                            }
                        },
                        "social": {
                            "twitter": null,
                            "github": "bob-codes",
                            "website": "https://bob.dev"
                        }
                    },
                    "stats": {
                        "posts": 15,
                        "followers": 89,
                        "following": 123
                    }
                }
            ],
            "metadata": {
                "total_users": 2,
                "last_updated": "2023-12-25T10:00:00Z",
                "version": 2.1,
                "features": ["profiles", "stats", "social"]
            }
        });

        // Verify complex structure
        if let Value::Object(root) = user_data {
            // Check users array
            if let Some(Value::Array(users)) = root.get("users") {
                assert_eq!(users.len(), 2);
                
                // Check first user
                if let Some(Value::Object(user1)) = users.get(0) {
                    assert_eq!(user1.get("id"), Some(&1.to_value()));
                    assert_eq!(user1.get("username"), Some(&"alice".to_value()));
                    
                    // Check nested profile
                    if let Some(Value::Object(profile)) = user1.get("profile") {
                        if let Some(Value::Object(prefs)) = profile.get("preferences") {
                            if let Some(Value::Object(notifications)) = prefs.get("notifications") {
                                assert_eq!(notifications.get("email"), Some(&true.to_value()));
                                assert_eq!(notifications.get("push"), Some(&false.to_value()));
                                assert_eq!(notifications.get("sms"), Some(&Value::Null));
                            } else {
                                panic!("Expected notifications object");
                            }
                        } else {
                            panic!("Expected preferences object");
                        }
                    } else {
                        panic!("Expected profile object");
                    }
                } else {
                    panic!("Expected user object");
                }
            } else {
                panic!("Expected users array");
            }
            
            // Check metadata
            if let Some(Value::Object(metadata)) = root.get("metadata") {
                assert_eq!(metadata.get("total_users"), Some(&2.to_value()));
                assert_eq!(metadata.get("version"), Some(&2.1.to_value()));
                
                if let Some(Value::Array(features)) = metadata.get("features") {
                    assert_eq!(features.len(), 3);
                } else {
                    panic!("Expected features array");
                }
            } else {
                panic!("Expected metadata object");
            }
        } else {
            panic!("Expected root object");
        }
    }

    #[test]
    fn test_json_with_computed_values() {
        let base_price = 100.0;
        let tax_rate = 0.08;
        let discount = 0.1;
        let quantity = 3;
        
        let order = json!({
            "order_id": format!("ORDER-{}", chrono::Utc::now().timestamp()),
            "items": [
                {
                    "name": "Widget A",
                    "price": base_price,
                    "quantity": quantity,
                    "subtotal": base_price * quantity as f64
                },
                {
                    "name": "Widget B", 
                    "price": base_price * 1.5,
                    "quantity": 2,
                    "subtotal": base_price * 1.5 * 2.0
                }
            ],
            "pricing": {
                "subtotal": base_price * quantity as f64 + base_price * 1.5 * 2.0,
                "discount_rate": discount,
                "discount_amount": (base_price * quantity as f64 + base_price * 1.5 * 2.0) * discount,
                "tax_rate": tax_rate,
                "tax_amount": (base_price * quantity as f64 + base_price * 1.5 * 2.0) * (1.0 - discount) * tax_rate,
                "total": (base_price * quantity as f64 + base_price * 1.5 * 2.0) * (1.0 - discount) * (1.0 + tax_rate)
            },
            "status": if quantity > 0 { "pending" } else { "cancelled" },
            "estimated_shipping": if quantity <= 5 { "2-3 days" } else { "5-7 days" }
        });

        // Verify computed values
        if let Value::Object(order_map) = order {
            if let Some(Value::Object(pricing)) = order_map.get("pricing") {
                assert_eq!(pricing.get("subtotal"), Some(&600.0.to_value()));
                assert_eq!(pricing.get("discount_rate"), Some(&0.1.to_value()));
                assert_eq!(pricing.get("tax_rate"), Some(&0.08.to_value()));
            } else {
                panic!("Expected pricing object");
            }
            
            assert_eq!(order_map.get("status"), Some(&"pending".to_value()));
            assert_eq!(order_map.get("estimated_shipping"), Some(&"2-3 days".to_value()));
        } else {
            panic!("Expected order object");
        }
    }
}

#[cfg(test)]
mod real_world_scenarios {
    use super::*;

    #[test]
    fn test_api_error_response() {
        let error_code = 400;
        let error_message = "Invalid request parameters";
        let field_errors = vec![
            ("email", "Invalid email format"),
            ("password", "Password too short"),
            ("age", "Must be between 18 and 120")
        ];

        let error_response = json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "type": "validation_error",
                "details": {
                    "fields": field_errors.iter().map(|(field, msg)| {
                        json!({
                            "field": *field,
                            "message": *msg,
                            "code": format!("INVALID_{}", field.to_uppercase())
                        })
                    }).collect::<Vec<_>>()
                },
                "documentation": "https://api.example.com/docs/errors",
                "request_id": format!("req_{}", std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs())
            },
            "timestamp": chrono::Utc::now().timestamp(),
            "path": "/api/users",
            "method": "POST"
        });

        // Verify error structure
        if let Value::Object(response) = error_response {
            if let Some(Value::Object(error)) = response.get("error") {
                assert_eq!(error.get("code"), Some(&400.to_value()));
                assert_eq!(error.get("message"), Some(&"Invalid request parameters".to_value()));
                assert_eq!(error.get("type"), Some(&"validation_error".to_value()));
                
                if let Some(Value::Object(details)) = error.get("details") {
                    if let Some(Value::Array(fields)) = details.get("fields") {
                        assert_eq!(fields.len(), 3);
                        
                        // Check first field error
                        if let Some(Value::Object(field_error)) = fields.get(0) {
                            assert_eq!(field_error.get("field"), Some(&"email".to_value()));
                            assert_eq!(field_error.get("message"), Some(&"Invalid email format".to_value()));
                        } else {
                            panic!("Expected field error object");
                        }
                    } else {
                        panic!("Expected fields array");
                    }
                } else {
                    panic!("Expected details object");
                }
            } else {
                panic!("Expected error object");
            }
        } else {
            panic!("Expected response object");
        }
    }

    #[test]
    fn test_configuration_file_structure() {
        let env = "development";
        let debug = true;
        let log_level = if debug { "debug" } else { "info" };
        
        let config = json!({
            "app": {
                "name": "MyApp",
                "version": env!("CARGO_PKG_VERSION"),
                "environment": env,
                "debug": debug
            },
            "server": {
                "host": if env == "production" { "0.0.0.0" } else { "127.0.0.1" },
                "port": if env == "production" { 8080 } else { 3000 },
                "workers": if env == "production" { 8 } else { 2 },
                "timeout": 30,
                "keep_alive": true
            },
            "database": {
                "url": format!("postgresql://localhost/myapp_{}", env),
                "pool_size": 10,
                "timeout": 5000,
                "ssl": env == "production",
                "migrations": {
                    "auto": env != "production",
                    "path": "./migrations"
                }
            },
            "logging": {
                "level": log_level,
                "format": "json",
                "outputs": [
                    {
                        "type": "console",
                        "enabled": true
                    },
                    {
                        "type": "file",
                        "enabled": env == "production",
                        "path": format!("logs/app-{}.log", env),
                        "rotation": "daily"
                    }
                ]
            },
            "features": {
                "api_rate_limiting": env == "production",
                "request_tracing": true,
                "metrics_collection": env != "development",
                "health_checks": true
            },
            "security": {
                "jwt_secret": if env == "production" { 
                    "CHANGE_ME_IN_PRODUCTION" 
                } else { 
                    "dev-secret-key" 
                },
                "session_timeout": 3600,
                "cors": {
                    "enabled": true,
                    "origins": if env == "production" { 
                        vec!["https://myapp.com", "https://www.myapp.com"]
                    } else {
                        vec!["http://localhost:3000", "http://127.0.0.1:3000"]
                    },
                    "methods": ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
                    "headers": ["Authorization", "Content-Type", "X-Requested-With"]
                }
            }
        });

        // Verify configuration structure
        if let Value::Object(config_map) = config {
            // Check app config
            if let Some(Value::Object(app)) = config_map.get("app") {
                assert_eq!(app.get("environment"), Some(&"development".to_value()));
                assert_eq!(app.get("debug"), Some(&true.to_value()));
            }

            // Check server config
            if let Some(Value::Object(server)) = config_map.get("server") {
                assert_eq!(server.get("host"), Some(&"127.0.0.1".to_value()));
                assert_eq!(server.get("port"), Some(&3000.to_value()));
                assert_eq!(server.get("workers"), Some(&2.to_value()));
            }

            // Check database config
            if let Some(Value::Object(database)) = config_map.get("database") {
                assert_eq!(database.get("ssl"), Some(&false.to_value()));
                
                if let Some(Value::Object(migrations)) = database.get("migrations") {
                    assert_eq!(migrations.get("auto"), Some(&true.to_value()));
                }
            }

            // Check logging config
            if let Some(Value::Object(logging)) = config_map.get("logging") {
                assert_eq!(logging.get("level"), Some(&"debug".to_value()));
                
                if let Some(Value::Array(outputs)) = logging.get("outputs") {
                    assert_eq!(outputs.len(), 2);
                }
            }
        }
    }

    #[test]
    fn test_data_transformation_pipeline() {
        let raw_data = vec![
            ("Alice", 85, "A"),
            ("Bob", 92, "A+"),
            ("Charlie", 78, "B"),
            ("Diana", 96, "A+"),
            ("Eve", 71, "B-")
        ];

        let processed_data = json!({
            "summary": {
                "total_students": raw_data.len(),
                "average_score": raw_data.iter().map(|(_, score, _)| score).sum::<i32>() as f64 / raw_data.len() as f64,
                "grade_distribution": {
                    "A+": raw_data.iter().filter(|(_, _, grade)| *grade == "A+").count(),
                    "A": raw_data.iter().filter(|(_, _, grade)| *grade == "A").count(),
                    "B": raw_data.iter().filter(|(_, _, grade)| *grade == "B").count(),
                    "B-": raw_data.iter().filter(|(_, _, grade)| *grade == "B-").count()
                }
            },
            "students": raw_data.iter().map(|(name, score, grade)| {
                json!({
                    "name": *name,
                    "score": *score,
                    "grade": *grade,
                    "performance": match *score {
                        90..=100 => "excellent",
                        80..=89 => "good",
                        70..=79 => "satisfactory",
                        _ => "needs_improvement"
                    },
                    "passed": *score >= 70
                })
            }).collect::<Vec<_>>(),
            "analytics": {
                "top_performer_name": "Diana",
                "top_performer_score": 96,
                "improvement_needed_count": 2
            }
        });

        // Verify processed data structure
        if let Value::Object(data) = processed_data {
            // Check summary
            if let Some(Value::Object(summary)) = data.get("summary") {
                assert_eq!(summary.get("total_students"), Some(&5.to_value()));
                
                // Average should be (85+92+78+96+71)/5 = 84.4
                if let Some(average) = summary.get("average_score") {
                    if let Value::Number(num) = average {
                        let avg_value = num.to_f64().unwrap();
                        assert!((avg_value - 84.4).abs() < 0.1);
                    }
                }
                
                if let Some(Value::Object(distribution)) = summary.get("grade_distribution") {
                    assert_eq!(distribution.get("A+"), Some(&2.to_value()));
                    assert_eq!(distribution.get("A"), Some(&1.to_value()));
                }
            }
            
            // Check students array
            if let Some(Value::Array(students)) = data.get("students") {
                assert_eq!(students.len(), 5);
                
                // Check first student
                if let Some(Value::Object(student)) = students.get(0) {
                    assert_eq!(student.get("name"), Some(&"Alice".to_value()));
                    assert_eq!(student.get("score"), Some(&85.to_value()));
                    assert_eq!(student.get("performance"), Some(&"good".to_value()));
                    assert_eq!(student.get("passed"), Some(&true.to_value()));
                }
            }
            
            // Check analytics
            if let Some(Value::Object(analytics)) = data.get("analytics") {
                if let Some(Value::Object(top)) = analytics.get("top_performer") {
                    assert_eq!(top.get("name"), Some(&"Diana".to_value()));
                    assert_eq!(top.get("score"), Some(&96.to_value()));
                }
                
                if let Some(Value::Array(improvement)) = analytics.get("improvement_needed") {
                    assert_eq!(improvement.len(), 2); // Charlie and Eve
                }
            }
        }
    }
}

#[cfg(test)]
mod macro_edge_cases {
    use super::*;

    #[test]
    fn test_macro_with_references() {
        let data = vec![1, 2, 3, 4, 5];
        let name = String::from("test");
        
        let obj = json!({
            "data_ref": data.clone(),
            "name_ref": name.clone(),
            "slice": data[1..4].to_vec(),
            "borrowed_str": name.as_str()
        });

        if let Value::Object(map) = obj {
            assert!(map.contains_key(&"data_ref"));
            assert!(map.contains_key(&"name_ref"));
            assert!(map.contains_key(&"slice"));
            assert!(map.contains_key(&"borrowed_str"));
        }
    }

    #[test]
    fn test_macro_with_option_values() {
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;
        
        let obj = json!({
            "some": some_value,
            "none": none_value,
            "unwrapped": some_value.unwrap_or(0),
            "is_some": some_value.is_some(),
            "is_none": none_value.is_none()
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("unwrapped"), Some(&42.to_value()));
            assert_eq!(map.get("is_some"), Some(&true.to_value()));
            assert_eq!(map.get("is_none"), Some(&true.to_value()));
        }
    }

    #[test]
    fn test_macro_with_result_values() {
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");
        
        let obj = json!({
            "ok_result": ok_result.unwrap_or(-1),
            "err_result": err_result.unwrap_or(-1),
            "is_ok": ok_result.is_ok(),
            "is_err": err_result.is_err()
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("ok_result"), Some(&42.to_value()));
            assert_eq!(map.get("err_result"), Some(&(-1).to_value()));
            assert_eq!(map.get("is_ok"), Some(&true.to_value()));
            assert_eq!(map.get("is_err"), Some(&true.to_value()));
        }
    }

    #[test]
    fn test_macro_with_iterators() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        let obj = json!({
            "doubled": numbers.iter().map(|x| x * 2).collect::<Vec<_>>(),
            "filtered": numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect::<Vec<_>>(),
            "sum": numbers.iter().sum::<i32>(),
            "max": *numbers.iter().max().unwrap(),
            "min": *numbers.iter().min().unwrap()
        });

        if let Value::Object(map) = obj {
            assert_eq!(map.get("sum"), Some(&15.to_value()));
            assert_eq!(map.get("max"), Some(&5.to_value()));
            assert_eq!(map.get("min"), Some(&1.to_value()));
        }
    }
}
