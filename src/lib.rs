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
//!   // `x_`, `_x` helpers for simple types
//!   assert_eq!(capitalize!(json!("FRED")), json!("Fred"));
//!   assert_eq!(x_capitalize!("FRED"), json!("Fred"));
//!   assert_eq!(capitalize_x!(json!("FRED")), "Fred".to_owned());
//!   assert_eq!(x_capitalize_x!("FRED"), "Fred".to_owned());
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

#[cfg(feature = "camel")]
extern crate paste;

#[cfg(feature = "lazy_static")]
extern crate lazy_static;

mod macros;

mod lib {
    pub use serde_json::{Value, Map, json};

    pub fn get_type_name<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }
}

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
