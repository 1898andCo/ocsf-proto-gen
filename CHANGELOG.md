# Changelog

## 0.1.1 — 2026-02-25

- Add missing OCSF string-derived types: `bytestring_t`, `file_hash_t`, `reg_key_path_t`
- Remove nonexistent `path_t` from type mapping
- Validate all 24 OCSF types against the v1.7.0 type hierarchy
- Add explicit tests for `timestamp_t` (int64, epoch ms) vs `datetime_t` (string, RFC 3339)
- Update README type mapping table with full OCSF type hierarchy and base types
- Fix test isolation: use unique temp directories per test to prevent parallel collisions in CI
- Add 8 integration tests covering full E2E pipeline
- Add CLAUDE.md with project overview and conventions
- Add release.yml workflow for tag-based crates.io publishing

## 0.1.0 — 2026-02-25

Initial release.

- Generate proto3 files from OCSF schema export API
- Support for all 83 OCSF 1.7.0 event classes
- Transitive object dependency resolution
- Per-class and shared-object enum generation with qualified type references
- Deprecated field skipping
- String-keyed enum detection (HTTP methods, TLP colors, CVSS depth)
- Extension-prefixed object handling (e.g., `win/win_service`)
- `json_t` maps to `string` (not `google.protobuf.Struct`)
- Deterministic output (byte-identical across runs)
- CLI with `download-schema` and `generate` subcommands
- Library API for programmatic use
