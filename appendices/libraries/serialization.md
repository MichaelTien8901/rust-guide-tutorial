---
layout: default
title: Serialization
parent: Libraries
grand_parent: Appendices
nav_order: 1
---

# Serialization Libraries

Libraries for converting Rust data structures to and from various formats.

## serde

The foundational serialization framework for Rust.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

### Basic Usage

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}
```

### Attributes

| Attribute | Purpose |
|-----------|---------|
| `#[serde(rename = "n")]` | Rename field in output |
| `#[serde(skip)]` | Skip field entirely |
| `#[serde(default)]` | Use Default if missing |
| `#[serde(flatten)]` | Flatten nested struct |
| `#[serde(with = "module")]` | Custom serialization |

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "serverName")]
    server_name: String,

    #[serde(default)]
    port: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}
```

## serde_json

JSON serialization and deserialization.

```toml
[dependencies]
serde_json = "1.0"
```

### Serialize/Deserialize

```rust
use serde_json;

let user = User { name: "Alice".into(), age: 30, email: "alice@example.com".into() };

// To JSON string
let json = serde_json::to_string(&user)?;
let json_pretty = serde_json::to_string_pretty(&user)?;

// From JSON string
let user: User = serde_json::from_str(&json)?;

// To/from bytes
let bytes = serde_json::to_vec(&user)?;
let user: User = serde_json::from_slice(&bytes)?;
```

### Dynamic JSON

```rust
use serde_json::{json, Value};

// Create JSON dynamically
let value = json!({
    "name": "Alice",
    "age": 30,
    "tags": ["admin", "user"]
});

// Access fields
let name = value["name"].as_str();
let age = value["age"].as_u64();

// Parse unknown JSON
let data: Value = serde_json::from_str(json_str)?;
```

## toml

TOML configuration file format.

```toml
[dependencies]
toml = "0.8"
```

```rust
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

// Parse TOML
let config: Config = toml::from_str(toml_str)?;

// Generate TOML
let toml_string = toml::to_string(&config)?;
let toml_pretty = toml::to_string_pretty(&config)?;
```

## serde_yaml

YAML serialization.

```toml
[dependencies]
serde_yaml = "0.9"
```

```rust
use serde_yaml;

let config: Config = serde_yaml::from_str(yaml_str)?;
let yaml_string = serde_yaml::to_string(&config)?;
```

## bincode

Compact binary serialization.

```toml
[dependencies]
bincode = "1.3"
```

```rust
use bincode;

// Serialize to bytes (compact, fast)
let bytes: Vec<u8> = bincode::serialize(&data)?;

// Deserialize from bytes
let data: MyStruct = bincode::deserialize(&bytes)?;

// With size limits
let config = bincode::config().limit(1024);
let bytes = config.serialize(&data)?;
```

## ciborium

CBOR (Concise Binary Object Representation) format.

```toml
[dependencies]
ciborium = "0.2"
```

```rust
use ciborium;

let mut bytes = Vec::new();
ciborium::into_writer(&data, &mut bytes)?;

let data: MyStruct = ciborium::from_reader(&bytes[..])?;
```

## rmp-serde

MessagePack serialization.

```toml
[dependencies]
rmp-serde = "1.1"
```

```rust
use rmp_serde;

let bytes = rmp_serde::to_vec(&data)?;
let data: MyStruct = rmp_serde::from_slice(&bytes)?;
```

## csv

CSV file handling.

```toml
[dependencies]
csv = "1.3"
```

```rust
use csv;

#[derive(Serialize, Deserialize)]
struct Record {
    name: String,
    value: f64,
}

// Read CSV
let mut reader = csv::Reader::from_path("data.csv")?;
for result in reader.deserialize() {
    let record: Record = result?;
    println!("{}: {}", record.name, record.value);
}

// Write CSV
let mut writer = csv::Writer::from_path("output.csv")?;
writer.serialize(Record { name: "test".into(), value: 42.0 })?;
writer.flush()?;
```

## Comparison

| Crate | Format | Size | Speed | Human Readable |
|-------|--------|------|-------|----------------|
| serde_json | JSON | Medium | Fast | Yes |
| toml | TOML | Medium | Fast | Yes |
| serde_yaml | YAML | Large | Medium | Yes |
| bincode | Binary | Small | Very Fast | No |
| rmp-serde | MsgPack | Small | Fast | No |
| ciborium | CBOR | Small | Fast | No |
| csv | CSV | Medium | Fast | Yes |

## Custom Serialization

```rust
use serde::{Serializer, Deserializer};

fn serialize_hex<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(bytes))
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    hex::decode(&s).map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(serialize_with = "serialize_hex", deserialize_with = "deserialize_hex")]
    key: Vec<u8>,
}
```

## Summary

| Use Case | Recommended |
|----------|-------------|
| Config files | toml |
| Web APIs | serde_json |
| Data exchange | serde_json |
| Binary storage | bincode |
| Cross-language binary | ciborium, rmp-serde |
| Tabular data | csv |
