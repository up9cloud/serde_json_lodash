//! # serde_json_lodash
//!
//! A library uses [lodash.js](https://lodash.com/docs) style functions to handle [serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html)

#![deny(missing_docs)]
#![deny(warnings)]

mod lib {
    pub use serde_json::{Value, json};
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

pub use array::*;
pub use collection::*;
pub use date::*;
pub use function::*;
pub use lang::*;
pub use math::*;
pub use number::*;
pub use object::*;
pub use seq::*;
pub use string::*;
pub use util::*;
pub use properties::*;
pub use methods::*;
