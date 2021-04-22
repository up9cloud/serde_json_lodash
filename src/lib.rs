//! # serde_json_lodash
//!
//! A library uses [lodash.js](https://lodash.com/docs) style functions to handle [serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html)

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
