
    use serde_json::{json, Value};
    use std::fs::{self, File};
    use std::io::Write;
    use std::thread;
    use config_ro::Config;

    fn create_temp_config(name: &str, content: &str) {
        fs::create_dir("configs");
        let mut file = File::create(format!("configs/{}.json", name)).unwrap();
        writeln!(file, "{}", content).unwrap();
    }
    fn delete_temp() {
        fs::remove_dir_all("configs");
    }

    #[test]
    fn test_config_new_loads_file() {
        let config_content = r#"{"key": "value"}"#;
        create_temp_config("test_config", config_content);

        let config = Config::new("test_config");

        assert_eq!(config.get::<String>("key").unwrap(), "value");
        delete_temp()
    }

    #[test]
    fn test_config_get_existing_key() {
        let config_content = r#"{"db_host": "localhost", "db_port": 5432,"nested":{"name":"test","id":2}}"#;
        create_temp_config("db_config", config_content);

        let config = Config::new("db_config");
        let db_host = config.get("db_host");
        let db_port = config.get("db_port");
        let name = config.get("nested.name");
        let id = config.get("nested.id");

        assert_eq!(name, Some(Value::String("test".to_string())));
        assert_eq!(id, Some(Value::Number(2.into())));
        assert_eq!(db_host, Some(Value::String("localhost".to_string())));
        assert_eq!(db_port, Some(Value::Number(5432.into())));
        delete_temp()
    }

    #[test]
    fn test_config_get_non_existing_key() {
        let config_content = r#"{"existing_key": "value"}"#;
        create_temp_config("test_config", config_content);

        let config = Config::new("test_config");
        let result = config.get("non_existing_key").unwrap_or(json!("default_value"));

        assert_eq!(result, json!("default_value"));
        delete_temp()
    }

    #[test]
    fn test_config_get_without_default() {
        let config_content = r#"{"existing_key": "value"}"#;
        create_temp_config("test_config", config_content);

        let config = Config::new("test_config");
        let result = config.get::<String>("non_existing_key");

        assert_eq!(result, None);
        delete_temp()
    }

    #[test]
    fn test_config_caching() {
        let config_content = r#"{"key": "value"}"#;
        create_temp_config("cache_test", config_content);

        // First load - should read from file
        let _config1 = Config::new("cache_test");

        // Modify the file
        let new_content = r#"{"key": "new_value"}"#;
        create_temp_config("cache_test", new_content);

        // Second load - should use cached version
        let config2 = Config::new("cache_test");
        let value = config2.get::<String>("key").unwrap();

        // Should still have old value from cache
        assert_eq!(value, "value".to_string());
        delete_temp()
    }

    #[test]
    #[should_panic(expected = "Failed to read config file:")]
    fn test_missing_config_file() {
        // Attempt to load non-existent config
        let _ = Config::new("non_existent_config");
    }

    #[test]
    #[should_panic(expected = "Invalid JSON format in")]
    fn test_invalid_json_config() {
        let config_content = r#"invalid json"#;
        create_temp_config("invalid_config", config_content);

        let _ = Config::new("invalid_config");
    }

    #[test]
    fn test_multiple_configs() {
        let config1_content = r#"{"key1": "value1"}"#;
        let config2_content = r#"{"key2": "value2"}"#;
        create_temp_config("config1", config1_content);
        create_temp_config("config2", config2_content);

        let config1 = Config::new("config1");
        let config2 = Config::new("config2");

        let value1 = config1.get::<String>("key1").unwrap();
        let value2 = config2.get::<String>("key2").unwrap();

        assert_eq!(value1, "value1".to_string());
        assert_eq!(value2, "value2".to_string());
        delete_temp()
    }
    // threads
    #[test]
    fn test_concurrent_config_access() {
        let config_content = r#"{"common_key": "common_value", "thread_specific": "initial"}"#;
        create_temp_config( "shared_config", config_content);

        Config::new("shared_config");
        let mut handles = vec![];

        // Spawn multiple threads to read the config
        for i in 0..10 {
            handles.push(thread::spawn(move || {
                let new_content = r#"{"key": "new_value"}"#;
                create_temp_config("shared_config", new_content);
                let value:String = Config::new("shared_config").get("common_key").unwrap();
                assert_eq!(value, "common_value");
                println!("Thread {} read common_key successfully", i);
            }));
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        delete_temp()
    }


    #[test]
    fn test_multiple_configs_concurrent_access() {
        let config1_content = r#"{"key1": "value1"}"#;
        let config2_content = r#"{"key2": "value2"}"#;
        create_temp_config( "config_a", config1_content);
        create_temp_config( "config_b", config2_content);


        let mut handles = vec![];

        // Spawn threads mixing access to both configs
        for i in 0..10 {
            handles.push(thread::spawn(move || {
                if i % 2 == 0 {
                    let value:String = Config::new("config_a").get("key1").unwrap();
                    assert_eq!(value, "value1");
                } else {
                    let value :String= Config::new("config_b").get("key2").unwrap();
                    assert_eq!(value, "value2");
                }
            }));
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        delete_temp()
    }

    #[test]
    fn test_high_contention_scenario() {
        let config_content = r#"{"contended_key": "value"}"#;
        create_temp_config( "high_contention_config", config_content);

        let mut handles = vec![];

        // Spawn many threads to create high contention
        for _ in 0..50 {
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let value :String= Config::new("high_contention_config").get("contended_key").unwrap();
                    assert_eq!(value, "value");
                }
            }));
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        delete_temp()
    }
