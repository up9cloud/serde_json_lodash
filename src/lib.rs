//! # serde_json_lodash
//!
//! A library uses [lodash.js](https://lodash.com/docs) style functions to handle [serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html)
//!
//! ## Usage
//!
//! ```rust
//! #[macro_use] extern crate serde_json_lodash;
//! use serde_json::json;
//!
//! fn main() {
//!     // Macro form: variadic / optional arguments, like the JS original
//!     assert_eq!(
//!         merge!(json!({"a": 1}), json!({"b": 2}), json!({"c": 3})),
//!         json!({"a": 1, "b": 2, "c": 3})
//!     );
//!
//!     // Function form: fixed arguments
//!     use serde_json_lodash::capitalize;
//!     assert_eq!(capitalize(json!("FRED")), json!("Fred"));
//!
//!     // A data argument can be a primitive instead of a `json!` value; the
//!     // `_x` form returns a primitive instead of a `Value`.
//!     assert_eq!(capitalize!("FRED"), json!("Fred"));
//!     assert_eq!(capitalize_x!("FRED"), "Fred".to_owned());
//! }
//! ```
//!
//! ## Naming: `_x` primitive-output helpers
//!
//! Everything is `serde_json::Value` in and out. A data argument may also be a
//! primitive (the base fn/macro are generic over `Into<Value>`), and every
//! function has a `_x` variant that returns a primitive instead of a `Value`:
//!
//! | Form           | Output   | Example (either arg form works)         |
//! | -------------- | -------- | --------------------------------------- |
//! | `capitalize`   | `Value`  | `capitalize("FRED")   // json!("Fred")` |
//! | `capitalize_x` | `String` | `capitalize_x("FRED") // "Fred"`        |
//!
//! Options that aren't data (sizes, indexes, pad chars) stay primitive
//! (`usize`, `isize`, `&str`); predicates use `Fn(&Value) -> bool`. Where a
//! result has no single primitive form (a collection, or a runtime-typed value
//! like `get`/`nth`), `_x` is an unimplemented `todo!()` marker. JSON has no
//! `undefined`, so functions that would return it return `Value::Null`.
//!
//! Every name exists as a `fn` and a macro, for both the base and `_x` form.
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
//!
//! ## Features
//!
//! No features are enabled by default (snake_case fns/macros only). Opt in as needed:
//!
//! | Feature       | Description |
//! | ------------- | ----------- |
//! | `alias`       | Aliasing machinery + snake_case lodash aliases (`first`, `entries`, `has_in`, …). Without it, use the canonical name (`head`, not `first`). Pulls in the optional [`paste`](https://crates.io/crates/paste) crate. |
//! | `camel`       | camelCase aliases (`isEmpty`, `hasIn`, …) and their `X`-suffixed `_x` forms (`isEmptyX`). Requires (and enables) `alias`. |
//! | `lazy_static` | Enables `unique_id` / `uniqueId`. |
//! | `all`         | `camel` + `lazy_static`. |
//!
//! Each alias re-exports the whole family (`fn`, macro, and both `_x` variants);
//! snake aliases keep the `_x` suffix (`has_in_x`), camelCase aliases use `X`
//! (`hasInX`).
//!
//! ## Iteratee shorthands
//!
//! Like lodash, the collection macros accept a shorthand in place of a
//! callback: an inline `json!` object is a partial deep match (`_.matches`), a
//! `json!([path, value])` pair is `_.matchesProperty`, and a string literal is
//! a `_.property` path — e.g. `filter!(users, json!({"active": true}))` or
//! `map!(users, "user")`. The combinators behind them are real functions too:
//! [`iteratee()`](fn@iteratee), [`matches()`](fn@matches),
//! [`matches_property()`](fn@matches_property) and [`property()`](fn@property)
//! return closures you can pass anywhere a callback is expected (useful when
//! the spec lives in a variable).
//!
//! The inline form is recognized **syntactically**: exactly the tokens
//! `json!(…)` or `serde_json::json!(…)` (the contents are expanded with the
//! bundled `serde_json`, so it works even without importing `json!`). Any
//! other spelling — a renamed crate path like `my_json::json!(…)`, or a
//! `Value` in a variable — is passed through as an expression, so wrap it in a
//! combinator instead: [`matches()`](fn@matches)/[`matches_property()`](fn@matches_property)
//! in predicate positions, [`iteratee()`](fn@iteratee)/[`property()`](fn@property)
//! in iteratee positions.
//!
//! ## What isn't ported
//!
//! Functions whose result is itself a **function** (`debounce`, `curry`,
//! `memoize`, `flow`, …) or that invoke object **methods** (`invoke`,
//! `invokeMap`, `create`) have no meaningful mapping onto `serde_json::Value`
//! and are intentionally left unimplemented. Each such stub is annotated in
//! the source with the reason. (`iteratee`, `matches`, `matchesProperty` and
//! `property` *are* ported — they return Rust closures.)

#![deny(missing_docs)]
#![deny(warnings)]

mod macros;

/// Re-exports of the [`serde_json`] types this crate operates on, so callers
/// can name them without a direct `serde_json` dependency.
pub mod lib {
    pub use serde_json::{Map, Number, Value, json};
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
mod methods;
mod number;
mod object;
mod properties;
mod seq;
mod string;
mod util;

mod lib_snake;
pub use lib_snake::*;

// Same fns as the crate root, but reachable through a module path, where the
// `#[macro_export]` macros (which are mounted at the crate root only) do not
// exist. The alias machinery imports `_x` fns through here: a plain
// `pub use $crate::head_x` would also grab the macro namespace, and
// macro-expanded `#[macro_export]` macros (`build_not_provided_x!` output)
// cannot be referred to by absolute paths.
#[doc(hidden)]
pub mod __fn {
    pub use crate::lib_snake::*;
}

#[cfg(feature = "camel")]
mod lib_camel;
#[cfg(feature = "camel")]
pub use lib_camel::*;
