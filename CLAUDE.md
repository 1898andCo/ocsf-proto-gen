# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

`ocsf-proto-gen` is a Rust CLI and library that generates Protocol Buffer (.proto) definitions from OCSF (Open Cybersecurity Schema Framework) JSON schema. It reads the OCSF export API and produces deterministic proto3 files for event classes, shared objects, and enums.

## Build & Test

```bash
cargo build                     # Build the binary
cargo test                      # Run all tests (unit + integration + doc)
cargo clippy -- -D warnings     # Lint (must pass clean)
cargo fmt --all -- --check      # Format check
cargo doc --no-deps             # Build documentation
cargo publish --dry-run         # Verify crates.io readiness
```

## Architecture

```
src/
├── lib.rs          # Public API: re-exports schema + codegen + type_map + error
├── main.rs         # CLI: download-schema and generate subcommands (clap)
├── error.rs        # Error types via thiserror (Error enum with Schema, ClassNotFound, Write, Read, Json, Download, Codegen)
├── schema.rs       # OCSF JSON serde types (OcsfSchema, OcsfClass, OcsfObject, OcsfAttribute, etc.) + loader + downloader
├── type_map.rs     # OCSF type → proto type mapping + name conversion utilities
└── codegen.rs      # Proto generation orchestrator: object graph resolution (BFS), event/object/enum file builders
```

### Data flow

```
OCSF export API (schema.ocsf.io/export/schema?version=X)
  → schema.json (cached locally)
    → schema::load_schema() → OcsfSchema
      → codegen::generate() → .proto files + enum-value-map.json
```

### Key design decisions

- **Schema source**: Uses the OCSF `/export/schema` API which returns fully-resolved classes (inheritance already applied). No need to implement OCSF's extends/include/profile merging.
- **Type mapping follows OCSF type hierarchy**: 6 primitives (`boolean_t`, `integer_t`, `long_t`, `float_t`, `string_t`, `json_t`) plus 18 derived types. `timestamp_t` derives from `long_t` → `int64` (epoch ms). `datetime_t` derives from `string_t` → `string` (RFC 3339). `port_t` derives from `integer_t` → `int32`. All other derived types inherit from `string_t` → `string`. See `type_map.rs` for the full table.
- **json_t → string**: Maps `json_t` to proto `string`, NOT `google.protobuf.Struct`. Struct breaks prost serde.
- **Deprecated fields**: Skipped entirely from output.
- **String-keyed enums**: OCSF has both integer-keyed (`"0": "Unknown"`) and string-keyed (`"GET": "Get"`) enums. Only integer-keyed become proto enums. Detected via `key.parse::<i32>().is_ok()`.
- **Extension prefixes**: OCSF extension objects use `win/win_service` format. Generator strips prefix for proto message names.
- **Field numbering**: Alphabetical sequential (1, 2, 3...). Deterministic across runs.
- **Enum type references**: Event fields with enums reference qualified enum types (e.g., `ocsf.v1_7_0.events.iam.enums.AUTHENTICATION_ACTIVITY_ID`), not primitive `int32`.

## Code Standards

- All public types and functions must have `///` doc comments
- Error handling: `thiserror` + `?` operator. No `panic!()` or `unwrap()` in non-test code
- Exception: `writeln!(string, ...).unwrap()` is acceptable — `fmt::Write` for `String` is infallible
- Tests use `unwrap()` freely
- Output must be deterministic: `BTreeMap` for all collections (sorted keys)

## Git Conventions

- Conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`
- Tag releases: `v0.1.0` format
- MIT licensed

## Testing

- **Unit tests**: In `src/type_map.rs` and `src/schema.rs` (13 tests) — test type mapping, name conversion, schema parsing
- **Integration tests**: In `tests/integration.rs` (8 tests) — end-to-end generation, content validation, determinism, error handling
- **Doc tests**: In `src/lib.rs` (1 test) — compile check for usage example

## OCSF Schema Reference

- Export API: `https://schema.ocsf.io/export/schema?version=1.7.0`
- Schema browser: `https://schema.ocsf.io/`
- GitHub: `https://github.com/ocsf/ocsf-schema`
- The export returns ~3.3MB JSON with all 83 event classes and 170 objects fully resolved

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with version, date, and changes
3. Commit: `chore: release v0.X.0`
4. Tag: `git tag v0.X.0`
5. Push: `git push origin main --tags`
6. GitHub Actions publishes to crates.io on tag push
