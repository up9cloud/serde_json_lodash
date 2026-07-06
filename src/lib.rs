//! # serde_json_lodash
//!
//! A library uses [lodash.js](https://lodash.com/docs) style functions to handle [serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html)
//!
//! ## Usage
//!
//! ```rust
//! #[macro_use] extern crate serde_json_lodash;
//! use serde_json::json;
//! fn main() {
//!   // macro style, optional parameters
//!   assert_eq!(
//!     merge!(json!({'a':1}), json!({'b':2}), json!({'c':3})),
//!     json!({'a': 1, 'b': 2, 'c': 3})
//!   );
//!
//!   // fn style, fixed parameters
//!   use serde_json_lodash::merge;
//!   assert_eq!(
//!     merge(json!({'a':1}), json!({'b':2})),
//!     json!({'a': 1, 'b': 2})
//!   );
//!
//!   // primitive in / primitive out: the base fn & macro accept a `&str`/`String`
//!   // primitive as well as a `json!` value; the `_x` form returns a primitive.
//!   assert_eq!(capitalize!(json!("FRED")), json!("Fred"));
//!   assert_eq!(capitalize!("FRED"), json!("Fred"));
//!   assert_eq!(capitalize_x!(json!("FRED")), "Fred".to_owned());
//!   assert_eq!(capitalize_x!("FRED"), "Fred".to_owned());
//! }
//! ```
//!
//! ## Primary macro vs. auxiliary forms
//!
//! For each lodash function the **primary macro** (e.g. [`capitalize!`]) is the
//! main, fully-documented entry point. Its doc has two sections:
//!
//! - `Examples:` — mirrors the lodash documentation for that function.
//! - `Additional cases:` — the interesting behavior: edge cases, empty/optional
//!   arguments, type coercion and JSON-specific quirks.
//!
//! Every other form is **auxiliary**: the plain function
//! ([`capitalize()`](fn@capitalize)) and the `_x` primitive-output helper with
//! its macro ([`capitalize_x()`](fn@capitalize_x), `capitalize_x!`). An
//! auxiliary item's `Additional cases:` is there only to **show how to call
//! that particular form** — one short example — not to re-cover the edge cases.
//!
//! For functions where an argument may reasonably be a primitive (e.g. a string
//! function's input), the base fn and macro are generic over
//! [`Into`]`<`[`Value`](crate::lib::Value)`>`, so a `&str`/`String`/number
//! primitive can be passed directly instead of wrapping it in `json!`. Where a
//! result has no single primitive form (a collection or a runtime-dynamic
//! value), the `_x` helper is a `todo!()` marker documenting that.
//!
//! So: to understand a function's behavior, read its primary macro; to see how
//! to call a specific form (fn vs macro, `Value` vs primitive), glance at that
//! item's one-line example.

#![deny(missing_docs)]
#![deny(warnings)]

mod macros;

/// Re-exports of the [`serde_json`] types this crate operates on, so callers
/// can name them without a direct `serde_json` dependency.
pub mod lib {
    pub use serde_json::{json, Value, Map, Number};
}

#[cfg(feature = "alias")]
#[doc(hidden)]
pub use paste;

#[doc(hidden)]
pub mod internal;

mod array;
mod collection;
mod date;
mod function;
mod lang;
mod math;
mod number;
mod object;
mod seq;
mod string;
mod util;
mod properties;
mod methods;

mod lib_snake;
pub use lib_snake::*;

#[cfg(feature = "camel")]
mod lib_camel;
#[cfg(feature = "camel")]
pub use lib_camel::*;
