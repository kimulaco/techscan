# Changelog

## 1.0.2 - 2025-09-21

### Fixes

- fix: percentage calculation for `lang` command [#30](https://github.com/kimulaco/techscan/issues/30)

## 1.0.1 - 2025-09-20

**No changes to the core library or CLI functionality.**

### Added

- Added Homebrew installation support: `brew tap kimulaco/techscan && brew install techscan`

## 1.0.0 - 2025-09-05

### Breaking changes

- Migrate language scanning to the `language` (`lang`) subcommand. [#5](https://github.com/kimulaco/techscan/issues/5)

  ```bash
  # before
  techscan scan_dir_path

  # After
  techscan lang scan_dir_path
  ```

### Feature

- Provide libraries. [#4](https://github.com/kimulaco/techscan/issues/4)
- Official support `--config` option. [#6](https://github.com/kimulaco/techscan/issues/6)
- Changed to return `TechScanError` when an error occurs. [#12](https://github.com/kimulaco/techscan/issues/12)
- Fixed add `--version` option. [#18](https://github.com/kimulaco/techscan/issues/18)

## [0.2.0] - 2025-08-23

### Feature

- Add language support ([#1](https://github.com/kimulaco/techscan/pull/1))
  - Astro (`.astro`)
  - C# (`.cs`)
  - COBOL (`.cbl`, `.cob`, `.cobol`)
  - Dart (`.dart`)
  - Elixir (`.ex`, `.exs`)
  - Haskell (`.hs`)
  - Kotlin (`.kt`, `.kts`)
  - Lua (`.lua`)
  - Objective-C (`.m`, `.mm`)
  - Perl (`.pl`, `.pm`)
  - R (`.r`, `.R`)
  - Scala (`.scala`, `.sc`)
  - Svelte (`.svelte`)
  - Swift (`.swift`)
  - Vue (`.vue`)

## 0.1.0 - 2025-08-22

### Feature

- `techscan` command: Extract the programming languages ​​in a directory and parse the percentages
  - Support for 14 programming languages: C, C++, CSS, Go, HTML, Java, JavaScript, PHP, Python, Ruby, Rust, SCSS, Shell, TypeScript
- `--reporter`option: Support multiple output formats. Table (default) and JSON output
- `--exclude`option: Exclusion file path patterns
