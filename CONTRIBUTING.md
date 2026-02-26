# Contributing to ocsf-proto-gen

## Development setup

```bash
git clone https://github.com/1898andCo/ocsf-proto-gen
cd ocsf-proto-gen
cargo build
cargo test
```

## Running checks

```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

## Code standards

- All public types and functions must have `///` doc comments
- No `panic!()` or `unwrap()` in non-test code, with one exception:
  `writeln!(string, ...)` is infallible (`fmt::Write` for `String` never fails),
  so `.unwrap()` is acceptable there
- Use `thiserror` for error types, propagate with `?`
- Tests use `unwrap()` freely — panics in tests are fine

## Pull requests

1. Fork the repo and create a branch
2. Make your changes
3. Run all checks (fmt, clippy, test)
4. Open a PR with a clear description

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
