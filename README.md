# techscan

`techscan` is a Rust CLI tool for analyzing and visualizing technology stacks in a directory.

## Install

### Cargo

```bash
cargo install techscan
```

## CLI

### Scan programming languages

```bash
techscan lang ./project

=== Scan Summary ===
┌─────────────┬────────────────────┐
│    Item     │       Value        │
├─────────────┼────────────────────┤
│ Directory   │ ./project.         │
│ Total Files │ 28                 │
└─────────────┴────────────────────┘

=== Language Statistics ===
┌────────────┬───────┬────────────┐
│  Language  │ Files │ Percentage │
├────────────┼───────┼────────────┤
│ TypeScript │ 4     │ 14.3%      │
│ JavaScript │ 4     │ 14.3%      │
│ C++        │ 3     │ 10.7%      │
│ HTML       │ 2     │ 7.1%       │
│ SCSS       │ 2     │ 7.1%       │
│ Shell      │ 1     │ 3.6%       │
│ PHP        │ 1     │ 3.6%       │
│ Go         │ 1     │ 3.6%       │
│ Rust       │ 1     │ 3.6%       │
│ Ruby       │ 1     │ 3.6%       │
│ Python     │ 1     │ 3.6%       │
│ CSS        │ 1     │ 3.6%       │
│ C          │ 1     │ 3.6%       │
└────────────┴───────┴────────────┘
```

#### Supported Languages

Astro, C, C++, C#, COBOL, CSS, Dart, Elixir, Go, Haskell, HTML, Java, JavaScript, Kotlin, Lua, Objective-C, Perl, PHP, Python, R, Ruby, Rust, Scala, SCSS, Shell, Svelte, Swift, TypeScript, Vue

**Note**: Frameworks and libraries with unique file extensions (e.g., `.vue`, `.svelte`, `.astro`) are analyzed as independent languages for technology stack analysis purposes.

Language addition requests are welcome through Issues or Pull Requests.

### CLI Options

#### Options

| Option | Short | Description | Default | Example |
|--------|-------|-------------|---------|---------|
| `--reporter` | `-r` | Output format: `table`, `json` | `table` | `--reporter json` |
| `--exclude` | `-e` | Exclude path patterns (can be used multiple times) | - | `--exclude "*.test.ts"` |

#### Examples

```bash
# Scan languages with table output (default)
techscan lang ./project

# Scan languages with JSON output
techscan lang ./project --reporter json

# Scan languages with exclude specific file patterns
techscan lang ./project --exclude "*.test.ts" --exclude "node_modules"

# Multiple excludes with short option
techscan lang ./project -e "*.test.ts" -e "*.spec.ts" -e "dist"
```

## Development

### Required

- **Rust**: >= 1.82.0

### Build & Debug

```bash
cargo build
cargo build --release

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

## License

[MIT License](LICENSE).
