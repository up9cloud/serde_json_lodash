/// **Not ported.** Invokes a function and captures thrown errors; no function to invoke in JSON.
pub fn attempt() {
    todo!()
}
/// **Not ported.** Binds object methods in place; JSON objects have no methods.
pub fn bind_all() {
    todo!()
}
#[doc(hidden)]
pub use bind_all as bindAll;
/// **Not ported.** Returns a function chosen from predicate/function pairs; not a Value.
pub fn cond() {
    todo!()
}
/// **Not ported.** Returns a predicate function from a spec; not a Value.
pub fn conforms() {
    todo!()
}
/// **Not ported.** Returns a function that returns a constant; not a Value.
pub fn constant() {
    todo!()
}
mod default_to;
pub use default_to::*;
/// **Not ported.** Composes functions left-to-right; not a Value.
pub fn flow() {
    todo!()
}
/// **Not ported.** Composes functions right-to-left; not a Value.
pub fn flow_right() {
    todo!()
}
#[doc(hidden)]
pub use flow_right as flowRight;
mod identity;
pub use identity::*;
/// **Not ported.** Returns a function implementing the lodash iteratee shorthand; not a Value.
pub fn iteratee() {
    todo!()
}
/// **Not ported.** Returns a predicate function from a source object; not a Value.
pub fn matches() {
    todo!()
}
/// **Not ported.** Returns a predicate function for a path/value; not a Value.
pub fn matches_property() {
    todo!()
}
#[doc(hidden)]
pub use matches_property as matchesProperty;
/// **Not ported.** Returns a function that invokes a method at a path; not a Value.
pub fn method() {
    todo!()
}
/// **Not ported.** Returns a function that invokes a method of an object; not a Value.
pub fn method_of() {
    todo!()
}
#[doc(hidden)]
pub use method_of as methodOf;
/// **Not ported.** Adds functions to an object/lodash; JSON objects hold no functions.
pub fn mixin() {
    todo!()
}
/// **Not ported.** Restores the global `_` binding; not applicable to a Rust library.
pub fn no_conflict() {
    todo!()
}
#[doc(hidden)]
pub use no_conflict as noConflict;
mod noop;
pub use noop::*;
/// **Not ported.** Returns a function selecting the nth argument; not a Value.
pub fn nth_arg() {
    todo!()
}
#[doc(hidden)]
pub use nth_arg as nthArg;
/// **Not ported.** Returns a function invoking several iteratees; not a Value.
pub fn over() {
    todo!()
}
/// **Not ported.** Returns a function AND-ing several predicates; not a Value.
pub fn over_every() {
    todo!()
}
#[doc(hidden)]
pub use over_every as overEvery;
/// **Not ported.** Returns a function OR-ing several predicates; not a Value.
pub fn over_some() {
    todo!()
}
#[doc(hidden)]
pub use over_some as overSome;
/// **Not ported.** Returns a getter function for a path; not a Value.
pub fn property() {
    todo!()
}
/// **Not ported.** Returns a getter function bound to an object; not a Value.
pub fn property_of() {
    todo!()
}
#[doc(hidden)]
pub use property_of as propertyOf;
mod range;
pub use range::*;
mod range_right;
pub use range_right::*;
/// **Not ported.** Creates a lodash bound to a context; not applicable to a Rust library.
pub fn run_in_context() {
    todo!()
}
#[doc(hidden)]
pub use run_in_context as runInContext;
mod stub_array;
pub use stub_array::*;
mod stub_false;
pub use stub_false::*;
mod stub_object;
pub use stub_object::*;
mod stub_string;
pub use stub_string::*;
mod stub_true;
pub use stub_true::*;
mod times;
pub use times::*;
mod to_path;
pub use to_path::*;
#[cfg(feature = "lazy_static")]
mod unique_id;
#[cfg(feature = "lazy_static")]
pub use unique_id::*;
