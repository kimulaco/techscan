# techscan

[![Latest Version](https://img.shields.io/github/v/release/kimulaco/techscan)](https://crates.io/crates/techscan)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

`techscan` is a CLI tool for analyzing and visualizing technology stacks in a directory.

`techscan` is build with Rust, providing high performance for analyzing large-scale projects such as monorepos and tracking technology stack migrations.

## Install

### Cargo

```bash
cargo install techscan
```

### Homebrew

```bash
brew tap kimulaco/techscan
brew install techscan
```

## CLI

### Scan programming languages

```bash
techscan lang .

Processing directory: .
=== Scan Summary ===
┌────────────────┬───────┐
│      Item      │ Value │
├────────────────┼───────┤
│ Directory      │   .   │
│ Total Files    │ 5160  │
│ Language Files │ 4179  │
│ Excluded Files │  981  │
└────────────────┴───────┘

=== Language Statistics ===
┌────────────┬───────┬────────────┐
│ Language   │ Files │ Percentage │
├────────────┼───────┼────────────┤
│ TypeScript │ 2978  │ 71.3%      │
│ JavaScript │ 926   │ 22.2%      │
│ HTML       │ 146   │ 3.5%       │
│ Svelte     │ 43    │ 1.0%       │
│ Vue        │ 41    │ 1.0%       │
│ Astro      │ 20    │ 0.5%       │
│ CSS        │ 16    │ 0.4%       │
│ Shell      │ 9     │ 0.2%       │
└────────────┴───────┴────────────┘
```

#### Supported Languages

Astro, C, C++, C#, COBOL, CSS, Dart, Elixir, Go, Haskell, HTML, Java, JavaScript, Kotlin, Lua, Objective-C, Perl, PHP, Python, R, Ruby, Rust, Scala, SCSS, Shell, Svelte, Swift, TypeScript, Vue

**Note**: Frameworks and libraries with unique file extensions (e.g., `.vue`, `.svelte`, `.astro`) are analyzed as independent languages for technology stack analysis purposes.

Language addition requests are welcome through Issues or Pull Requests.

### CLI Arguments

CLI arguments take precedence over configuration file options.

#### Options

| Option       | Short | Description                                        | Default | Example                  |
| ------------ | ----- | -------------------------------------------------- | ------- | ------------------------ |
| `--reporter` | `-r`  | Output format: `table`, `json`                     | `table` | `--reporter json`        |
| `--exclude`  | `-e`  | Exclude path patterns (can be used multiple times) | -       | `--exclude "*.test.ts"`  |
| `--config`   | `-c`  | Configuration file path                            | -       | `--config techscan.json` |
| `--version`  |       | Display techscan version                           | -       | `--version`              |

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

# Using configuration file
techscan lang ./project --config techscan.json

# Configuration file with CLI option override
techscan lang ./project --config techscan.json --exclude "node_modules"
```

### CLI Configuration File

`techscan` supports configuration files in JSON (`.json`, `.json5`), YAML (`.yaml`, `.yml`), and TOML (`.toml`) formats to set default options.

#### Options

| Setting    | Type             | Description                            | Default     | Example                |
| ---------- | ---------------- | -------------------------------------- | ----------- | ---------------------- |
| `exclude`  | Array of strings | File patterns to exclude from analysis | `[]` (none) | `["*.test.*", "dist"]` |
| `reporter` | String           | Output format (`"table"` or `"json"`)  | `"table"`   | `"json"`               |

#### Examples

##### JSON Format

```json
{
  "exclude": ["*.test.*", "dist"],
  "reporter": "json"
}
```

##### YAML Format

```yaml
exclude:
  - "*.test.*"
  - "dist"
reporter: "json"
```

##### TOML Format

```toml
exclude = ["*.test.*", "dist"]
reporter = "json"
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
