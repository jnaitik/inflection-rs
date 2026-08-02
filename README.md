# Inflection (Rust Port)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/jpvanhal/inflection)
[![Crates.io](https://img.shields.io/crates/v/inflection.svg)](https://crates.io/crates/inflection)

A robust, idiomatic Rust port of the popular Python [`inflection`](https://github.com/jpvanhal/inflection) library (which itself ports Ruby on Rails' `ActiveSupport::Inflector`). 

Created for the **Port Mortem 2026 Code Resurrection Hackathon**.

This library transforms English words into various forms. You can pluralize, singularize, turn CamelCase into snake_case, generate URL-friendly slugs, and more. 

## Features

- **100% Behavioral Parity:** Passes the entire original Python test suite.
- **Zero Unsafe:** Built using purely safe Rust.
- **Thread-Safe & Lock-Free:** Global inflection rules are safely compiled into static, immutable `LazyLock` structures, avoiding the mutable-global concurrency bugs of the original Python code.
- **Bug Catcher Patches:** We fixed several known subtle bugs from the original (e.g. empty string panics, regex anchor omissions, and case-sensitivity interactions). See [DECISIONS.md](DECISIONS.md) for details.
- **CLI Included:** Comes with a full-featured CLI wrapper via `clap`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
inflection = "0.1"
```

## Usage

### As a Library

```rust
use inflection::*;

// Pluralize / Singularize
assert_eq!(pluralize("octopus"), "octopi");
assert_eq!(singularize("axes"), "axis");

// Camelize / Underscore
assert_eq!(camelize("device_type", true), "DeviceType");
assert_eq!(camelize("device_type", false), "deviceType");
assert_eq!(underscore("DeviceType"), "device_type");

// Dasherize / Humanize / Titleize
assert_eq!(dasherize("puni_puni"), "puni-puni");
assert_eq!(humanize("employee_salary"), "Employee salary");
assert_eq!(titleize("man from the boondocks"), "Man From The Boondocks");

// URL Parameterize (Slugs)
assert_eq!(parameterize("Donald E. Knuth", "-"), "donald-e-knuth");

// Ordinals
assert_eq!(ordinal(1002), "nd");
assert_eq!(ordinalize(1002), "1002nd");
```

### As a CLI Tool

Install the CLI:
```bash
cargo install --path ./inflection-cli
```

Use the CLI:
```bash
$ inflection pluralize "matrix"
matrices

$ inflection camelize "active_record"
ActiveRecord

$ inflection camelize "active_record" --lower
activeRecord

$ inflection parameterize "Donald E. Knuth"
donald-e-knuth
```

## Supported Transformations
- `camelize(word, uppercase_first_letter)`
- `dasherize(word)`
- `humanize(word)`
- `ordinal(number)`
- `ordinalize(number)`
- `parameterize(word, separator)`
- `pluralize(word)`
- `singularize(word)`
- `tableize(word)`
- `titleize(word)`
- `transliterate(word)`
- `underscore(word)`

## Repository Structure

- `inflection/`: The core inflection library.
- `inflection-cli/`: The command-line interface wrapper.
- `original/`: Leftover original python files for reference.

## Architecture & Decisions

Please see [DECISIONS.md](DECISIONS.md) for an in-depth breakdown of how Python's regexes and stateful architecture were adapted to idiomatic, safe Rust.
