# tech-scan

**This project is still incomplete and under development.**

`tech-scan` is a Rust CLI tool for analyzing and visualizing technology stacks in a directory.

## Development

### Required

- **Rust**: >= 1.70

### Build & Debug

```bash
cargo build

cargo run <DIR> [OPTIONS]
```

### Quality check

```bash
cargo check

cargo clippy

cargo fmt
cargo fmt --check
```

### Testing

```bash
# testing all file
cargo test

# testing one file
cargo test language_config

# coverage
cargo llvm-cov --lib
```
