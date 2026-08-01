//! # inflection
//!
//! A Rust port of the Python [`inflection`](https://github.com/jpvanhal/inflection) library,
//! which is itself a port of Ruby on Rails' `ActiveSupport::Inflector`.
//!
//! This crate provides functions for transforming English words between
//! singular/plural forms, CamelCase/snake_case conversions, URL-friendly slugs,
//! and more.
//!
//! ## Design Philosophy
//!
//! - **Zero `unsafe` code** — entirely safe Rust
//! - **Thread-safe** — all inflection rules are immutable statics (unlike the
//!   Python original which uses mutable global state)
//! - **Behavioral equivalence** — preserves the original Python library's behavior
//!   as closely as possible, with documented bug fixes
//!
//! ## Quick Start
//!
//! ```rust
//! use inflection::{camelize, underscore, pluralize, singularize};
//!
//! assert_eq!(camelize("device_type", true), "DeviceType");
//! assert_eq!(underscore("DeviceType"), "device_type");
//! assert_eq!(pluralize("octopus"), "octopi");
//! assert_eq!(singularize("octopi"), "octopus");
//! ```

mod cases;
mod inflections;
mod numbers;
mod parameterize;

pub use cases::{camelize, dasherize, humanize, titleize, underscore};
pub use inflections::{pluralize, singularize, tableize};
pub use numbers::{ordinal, ordinalize};
pub use parameterize::{parameterize, transliterate};
