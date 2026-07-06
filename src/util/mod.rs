/// **Not ported.** Invokes a function and captures thrown errors; no function to invoke in JSON.
pub fn attempt() {
    todo!()
}
/// **Not ported.** Binds object methods in place; JSON objects have no methods.
pub fn bind_all() {
    todo!()
}
build_link_fn!(bindAll, bind_all);
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
build_link_fn!(flowRight, flow_right);
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
build_link_fn!(matchesProperty, matches_property);
/// **Not ported.** Returns a function that invokes a method at a path; not a Value.
pub fn method() {
    todo!()
}
/// **Not ported.** Returns a function that invokes a method of an object; not a Value.
pub fn method_of() {
    todo!()
}
build_link_fn!(methodOf, method_of);
/// **Not ported.** Adds functions to an object/lodash; JSON objects hold no functions.
pub fn mixin() {
    todo!()
}
/// **Not ported.** Restores the global `_` binding; not applicable to a Rust library.
pub fn no_conflict() {
    todo!()
}
build_link_fn!(noConflict, no_conflict);
mod noop;
pub use noop::*;
/// **Not ported.** Returns a function selecting the nth argument; not a Value.
pub fn nth_arg() {
    todo!()
}
build_link_fn!(nthArg, nth_arg);
/// **Not ported.** Returns a function invoking several iteratees; not a Value.
pub fn over() {
    todo!()
}
/// **Not ported.** Returns a function AND-ing several predicates; not a Value.
pub fn over_every() {
    todo!()
}
build_link_fn!(overEvery, over_every);
/// **Not ported.** Returns a function OR-ing several predicates; not a Value.
pub fn over_some() {
    todo!()
}
build_link_fn!(overSome, over_some);
/// **Not ported.** Returns a getter function for a path; not a Value.
pub fn property() {
    todo!()
}
/// **Not ported.** Returns a getter function bound to an object; not a Value.
pub fn property_of() {
    todo!()
}
build_link_fn!(propertyOf, property_of);
mod range;
pub use range::*;
mod range_right;
pub use range_right::*;
/// **Not ported.** Creates a lodash bound to a context; not applicable to a Rust library.
pub fn run_in_context() {
    todo!()
}
build_link_fn!(runInContext, run_in_context);
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
