# ocsf-proto-gen

Generate Protocol Buffer definitions from [OCSF](https://schema.ocsf.io/) (Open Cybersecurity Schema Framework) JSON schema.

## What it does

`ocsf-proto-gen` reads the OCSF schema export and generates deterministic `.proto` files for selected event classes, their transitive object dependencies, and enum definitions. The output is ready for compilation with `protoc`, `prost-build`, or any proto3-compatible toolchain.

## Why not ocsf-tool?

The community [ocsf-tool](https://github.com/valllabh/ocsf-tool) (Go CLI) has several issues that this tool fixes:

| Issue | ocsf-tool | ocsf-proto-gen |
|-------|-----------|----------------|
| Deprecated fields | Emits invalid `unknown` type | Skips deprecated attributes |
| Untyped fields (`json_t`) | Uses `google.protobuf.Struct` (breaks prost serde) | Emits `string` |
| String-keyed enums | Not handled | Detected and excluded from proto enums |
| Enum type references | Fields use primitive `int32` | Fields reference qualified enum types |
| Output determinism | Comment ordering varies between runs | Byte-identical output |
| Version pinning | Tracks `main` branch | Explicit `--ocsf-version` |
| Language | Go | Rust |

## Install

```bash
cargo install ocsf-proto-gen
```

Or build from source:

```bash
git clone https://github.com/1898andCo/ocsf-proto-gen
cd ocsf-proto-gen
cargo install --path .
```

## Quick start

```bash
# 1. Download the OCSF schema (cached locally)
ocsf-proto-gen download-schema --ocsf-version 1.7.0 --output-dir schema

# 2. Generate protos for selected event classes
ocsf-proto-gen generate \
    --ocsf-version 1.7.0 \
    --classes authentication,security_finding,network_activity \
    --output-dir proto \
    --schema-dir schema

# 3. Verify with protoc
protoc --proto_path=proto proto/ocsf/v1_7_0/events/iam/iam.proto --descriptor_set_out=/dev/null
```

Generate all 83 OCSF event classes:

```bash
ocsf-proto-gen generate --classes all --output-dir proto --schema-dir schema
```

## Output structure

```
proto/ocsf/v1_7_0/
├── enum-value-map.json                    # Reference: enum name → integer value
├── events/
│   ├── findings/
│   │   ├── enums/enums.proto              # SecurityFinding-specific enums
│   │   └── findings.proto                 # SecurityFinding message
│   ├── iam/
│   │   ├── enums/enums.proto              # Authentication-specific enums
│   │   └── iam.proto                      # Authentication message
│   └── network/
│       ├── enums/enums.proto              # NetworkActivity-specific enums
│       └── network.proto                  # NetworkActivity message
└── objects/
    ├── enums/enums.proto                  # Shared object enums
    └── objects.proto                      # All referenced object messages
```

## CLI reference

### `download-schema`

Download the OCSF schema export and cache locally.

```
ocsf-proto-gen download-schema [OPTIONS]

Options:
    --ocsf-version <VERSION>     OCSF version [default: 1.7.0]
    --output-dir <DIR>           Output directory [default: .]
    --schema-url <URL>           Schema API URL [env: OCSF_SCHEMA_URL]
```

### `generate`

Generate `.proto` files from a cached schema.

```
ocsf-proto-gen generate [OPTIONS] --classes <CLASSES>

Options:
    --ocsf-version <VERSION>     OCSF version [default: 1.7.0]
    --classes <CLASSES>           Comma-separated class names, or "all"
    --output-dir <DIR>           Output directory [default: .]
    --schema-dir <DIR>           Schema cache directory [default: .]
    -q, --quiet                  Suppress non-error output
```

## Library usage

```rust
use std::path::Path;

let schema = ocsf_proto_gen::schema::load_schema(Path::new("schema/1.7.0/schema.json"))?;
let stats = ocsf_proto_gen::codegen::generate(
    &schema,
    &["authentication".into(), "security_finding".into()],
    Path::new("proto/"),
)?;
println!("Generated {} classes, {} objects", stats.classes_generated, stats.objects_generated);
```

## OCSF type mapping

OCSF defines 24 types organized in a hierarchy. All mappings follow the OCSF type definitions:

| OCSF type | Proto type | OCSF base type | Notes |
|-----------|-----------|----------------|-------|
| `boolean_t` | `bool` | primitive | |
| `integer_t` | `int32` | primitive | Signed 32-bit |
| `long_t` | `int64` | primitive | Signed 64-bit |
| `float_t` | `double` | primitive | 64-bit float |
| `string_t` | `string` | primitive | UTF-8 |
| `json_t` | `string` | primitive | NOT `google.protobuf.Struct` |
| `timestamp_t` | `int64` | `long_t` | Epoch milliseconds |
| `port_t` | `int32` | `integer_t` | Range 0-65535 |
| `datetime_t` | `string` | `string_t` | RFC 3339 (e.g., `2024-09-10T23:20:50.520Z`) |
| `hostname_t`, `ip_t`, `mac_t`, `url_t`, `email_t`, `uuid_t`, `file_name_t`, `file_path_t`, `file_hash_t`, `process_name_t`, `resource_uid_t`, `username_t`, `subnet_t`, `bytestring_t`, `reg_key_path_t` | `string` | `string_t` | All string-derived types |
| Object references | Qualified message type | — | e.g., `ocsf.v1_7_0.objects.User` |
| Integer-keyed enums | Qualified enum type | — | e.g., `AUTHENTICATION_ACTIVITY_ID` |
| String-keyed enums | `string` | — | Not valid proto enums (e.g., HTTP methods) |

## Features

- `download` (default) — enables the `download-schema` command (adds `reqwest` + `tokio` deps)

To use as a library without network dependencies:

```toml
[dependencies]
ocsf-proto-gen = { version = "0.1", default-features = false }
```

## License

MIT
