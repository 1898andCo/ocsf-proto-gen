# Changelog

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
