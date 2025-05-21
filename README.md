# Config Manager

[![Crates.io](https://img.shields.io/crates/v/config-ro)](https://crates.io/crates/config-ro)
[![Documentation](https://docs.rs/config-ro/badge.svg)](https://docs.rs/config-ro)
[![License](https://img.shields.io/crates/l/config-ro)](LICENSE-MIT)


A thread-safe configuration manager for Rust applications that loads and caches JSON configuration files.

## Features

- **Thread-safe caching**: Configurations are loaded once and cached for subsequent accesses
- **Concurrent access**: Safe for use across multiple threads with `RwLock` synchronization
- **Type-safe retrieval**: Supports deserializing configuration values into Rust types
- **JSON-based**: Configuration files are stored in JSON format

## Usage

### Basic Usage

1. Create a JSON configuration file in the `configs/` directory (e.g., `configs/app.json`):

```json
{
  "db_host": "localhost",
  "db_port": 5432,
  "debug_mode": true
}
```

2. Use the configuration in your Rust code:

```rust
use config_ro::Config;

let config = Config::new("app");
let db_host: String = config.get("db_host").unwrap();
let db_port: u16 = config.get("db_port").unwrap();
let debug_mode: bool = config.get("debug_mode").unwrap();
```

### Advanced Usage

```rust
// Create multiple configuration instances
let app_config = Config::new("app");
let db_config = Config::new("database");

// Get nested values (if your JSON has nested objects)
let nested_value = app_config.get::<String>("nested.key").unwrap();

// Handle missing values
match app_config.get::<String>("optional_key") {
    Some(value) => println!("Got value: {}", value),
    None => println!("Using default value"),
}
```

## API Reference

### `Config::new(name: &str) -> Config`

Creates a new configuration instance for the given name. The configuration is loaded from `configs/{name}.json` if not already cached.

### `config.get<T: DeserializeOwned>(key: &str) -> Option<T>`

Retrieves a configuration value by key, attempting to deserialize it into type `T`.

## Thread Safety

The configuration manager is designed for concurrent access:

- Configuration loading is synchronized
- Multiple threads can read configurations simultaneously
- Configuration updates are exclusive (blocking other accesses during write)


## Example Configuration File

```json
{
  "app_name": "My Application",
  "version": "1.0.0",
  "database": {
    "host": "db.example.com",
    "port": 5432,
    "credentials": {
      "username": "admin",
      "password": "secret"
    }
  },
  "features": {
    "experimental": true,
    "logging": "verbose"
  }
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a PR for:
- New features
- Performance improvements
- Bug fixes


## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.