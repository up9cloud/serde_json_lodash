/// **Not ported.** Invokes a function and captures thrown errors; no function to invoke in JSON.
pub fn attempt() {
    todo!()
}
/// Based on [attempt()]
#[macro_export]
macro_rules! attempt {
    ($($t:tt)*) => {
        $crate::attempt()
    };
}
/// **Not ported.** Invokes a function and captures thrown errors; no function to invoke in JSON.
pub fn attempt_x() {
    todo!()
}
/// Based on [attempt_x()]
#[macro_export]
macro_rules! attempt_x {
    ($($t:tt)*) => {
        $crate::attempt_x()
    };
}
/// **Not ported.** Binds object methods in place; JSON objects have no methods.
pub fn bind_all() {
    todo!()
}
/// Based on [bind_all()]
#[macro_export]
macro_rules! bind_all {
    ($($t:tt)*) => {
        $crate::bind_all()
    };
}
/// **Not ported.** Binds object methods in place; JSON objects have no methods.
pub fn bind_all_x() {
    todo!()
}
/// Based on [bind_all_x()]
#[macro_export]
macro_rules! bind_all_x {
    ($($t:tt)*) => {
        $crate::bind_all_x()
    };
}
build_link!(bindAll, bind_all);
/// **Not ported.** Returns a function chosen from predicate/function pairs; not a Value.
pub fn cond() {
    todo!()
}
/// Based on [cond()]
#[macro_export]
macro_rules! cond {
    ($($t:tt)*) => {
        $crate::cond()
    };
}
/// **Not ported.** Returns a function chosen from predicate/function pairs; not a Value.
pub fn cond_x() {
    todo!()
}
/// Based on [cond_x()]
#[macro_export]
macro_rules! cond_x {
    ($($t:tt)*) => {
        $crate::cond_x()
    };
}
/// **Not ported.** Returns a predicate function from a spec; not a Value.
pub fn conforms() {
    todo!()
}
/// Based on [conforms()]
#[macro_export]
macro_rules! conforms {
    ($($t:tt)*) => {
        $crate::conforms()
    };
}
/// **Not ported.** Returns a predicate function from a spec; not a Value.
pub fn conforms_x() {
    todo!()
}
/// Based on [conforms_x()]
#[macro_export]
macro_rules! conforms_x {
    ($($t:tt)*) => {
        $crate::conforms_x()
    };
}
/// **Not ported.** Returns a function that returns a constant; not a Value.
pub fn constant() {
    todo!()
}
/// Based on [constant()]
#[macro_export]
macro_rules! constant {
    ($($t:tt)*) => {
        $crate::constant()
    };
}
/// **Not ported.** Returns a function that returns a constant; not a Value.
pub fn constant_x() {
    todo!()
}
/// Based on [constant_x()]
#[macro_export]
macro_rules! constant_x {
    ($($t:tt)*) => {
        $crate::constant_x()
    };
}
mod default_to;
pub use default_to::*;
/// **Not ported.** Composes functions left-to-right; not a Value.
pub fn flow() {
    todo!()
}
/// Based on [flow()]
#[macro_export]
macro_rules! flow {
    ($($t:tt)*) => {
        $crate::flow()
    };
}
/// **Not ported.** Composes functions left-to-right; not a Value.
pub fn flow_x() {
    todo!()
}
/// Based on [flow_x()]
#[macro_export]
macro_rules! flow_x {
    ($($t:tt)*) => {
        $crate::flow_x()
    };
}
/// **Not ported.** Composes functions right-to-left; not a Value.
pub fn flow_right() {
    todo!()
}
/// Based on [flow_right()]
#[macro_export]
macro_rules! flow_right {
    ($($t:tt)*) => {
        $crate::flow_right()
    };
}
/// **Not ported.** Composes functions right-to-left; not a Value.
pub fn flow_right_x() {
    todo!()
}
/// Based on [flow_right_x()]
#[macro_export]
macro_rules! flow_right_x {
    ($($t:tt)*) => {
        $crate::flow_right_x()
    };
}
build_link!(flowRight, flow_right);
mod identity;
pub use identity::*;
/// **Not ported.** Returns a function implementing the lodash iteratee shorthand; not a Value.
pub fn iteratee() {
    todo!()
}
/// Based on [iteratee()]
#[macro_export]
macro_rules! iteratee {
    ($($t:tt)*) => {
        $crate::iteratee()
    };
}
/// **Not ported.** Returns a function implementing the lodash iteratee shorthand; not a Value.
pub fn iteratee_x() {
    todo!()
}
/// Based on [iteratee_x()]
#[macro_export]
macro_rules! iteratee_x {
    ($($t:tt)*) => {
        $crate::iteratee_x()
    };
}
/// **Not ported.** Returns a predicate function from a source object; not a Value.
pub fn matches() {
    todo!()
}
/// Based on [matches()]
#[macro_export]
macro_rules! matches {
    ($($t:tt)*) => {
        $crate::matches()
    };
}
/// **Not ported.** Returns a predicate function from a source object; not a Value.
pub fn matches_x() {
    todo!()
}
/// Based on [matches_x()]
#[macro_export]
macro_rules! matches_x {
    ($($t:tt)*) => {
        $crate::matches_x()
    };
}
/// **Not ported.** Returns a predicate function for a path/value; not a Value.
pub fn matches_property() {
    todo!()
}
/// Based on [matches_property()]
#[macro_export]
macro_rules! matches_property {
    ($($t:tt)*) => {
        $crate::matches_property()
    };
}
/// **Not ported.** Returns a predicate function for a path/value; not a Value.
pub fn matches_property_x() {
    todo!()
}
/// Based on [matches_property_x()]
#[macro_export]
macro_rules! matches_property_x {
    ($($t:tt)*) => {
        $crate::matches_property_x()
    };
}
build_link!(matchesProperty, matches_property);
/// **Not ported.** Returns a function that invokes a method at a path; not a Value.
pub fn method() {
    todo!()
}
/// Based on [method()]
#[macro_export]
macro_rules! method {
    ($($t:tt)*) => {
        $crate::method()
    };
}
/// **Not ported.** Returns a function that invokes a method at a path; not a Value.
pub fn method_x() {
    todo!()
}
/// Based on [method_x()]
#[macro_export]
macro_rules! method_x {
    ($($t:tt)*) => {
        $crate::method_x()
    };
}
/// **Not ported.** Returns a function that invokes a method of an object; not a Value.
pub fn method_of() {
    todo!()
}
/// Based on [method_of()]
#[macro_export]
macro_rules! method_of {
    ($($t:tt)*) => {
        $crate::method_of()
    };
}
/// **Not ported.** Returns a function that invokes a method of an object; not a Value.
pub fn method_of_x() {
    todo!()
}
/// Based on [method_of_x()]
#[macro_export]
macro_rules! method_of_x {
    ($($t:tt)*) => {
        $crate::method_of_x()
    };
}
build_link!(methodOf, method_of);
/// **Not ported.** Adds functions to an object/lodash; JSON objects hold no functions.
pub fn mixin() {
    todo!()
}
/// Based on [mixin()]
#[macro_export]
macro_rules! mixin {
    ($($t:tt)*) => {
        $crate::mixin()
    };
}
/// **Not ported.** Adds functions to an object/lodash; JSON objects hold no functions.
pub fn mixin_x() {
    todo!()
}
/// Based on [mixin_x()]
#[macro_export]
macro_rules! mixin_x {
    ($($t:tt)*) => {
        $crate::mixin_x()
    };
}
/// **Not ported.** Restores the global `_` binding; not applicable to a Rust library.
pub fn no_conflict() {
    todo!()
}
/// Based on [no_conflict()]
#[macro_export]
macro_rules! no_conflict {
    ($($t:tt)*) => {
        $crate::no_conflict()
    };
}
/// **Not ported.** Restores the global `_` binding; not applicable to a Rust library.
pub fn no_conflict_x() {
    todo!()
}
/// Based on [no_conflict_x()]
#[macro_export]
macro_rules! no_conflict_x {
    ($($t:tt)*) => {
        $crate::no_conflict_x()
    };
}
build_link!(noConflict, no_conflict);
mod noop;
pub use noop::*;
/// **Not ported.** Returns a function selecting the nth argument; not a Value.
pub fn nth_arg() {
    todo!()
}
/// Based on [nth_arg()]
#[macro_export]
macro_rules! nth_arg {
    ($($t:tt)*) => {
        $crate::nth_arg()
    };
}
/// **Not ported.** Returns a function selecting the nth argument; not a Value.
pub fn nth_arg_x() {
    todo!()
}
/// Based on [nth_arg_x()]
#[macro_export]
macro_rules! nth_arg_x {
    ($($t:tt)*) => {
        $crate::nth_arg_x()
    };
}
build_link!(nthArg, nth_arg);
/// **Not ported.** Returns a function invoking several iteratees; not a Value.
pub fn over() {
    todo!()
}
/// Based on [over()]
#[macro_export]
macro_rules! over {
    ($($t:tt)*) => {
        $crate::over()
    };
}
/// **Not ported.** Returns a function invoking several iteratees; not a Value.
pub fn over_x() {
    todo!()
}
/// Based on [over_x()]
#[macro_export]
macro_rules! over_x {
    ($($t:tt)*) => {
        $crate::over_x()
    };
}
/// **Not ported.** Returns a function AND-ing several predicates; not a Value.
pub fn over_every() {
    todo!()
}
/// Based on [over_every()]
#[macro_export]
macro_rules! over_every {
    ($($t:tt)*) => {
        $crate::over_every()
    };
}
/// **Not ported.** Returns a function AND-ing several predicates; not a Value.
pub fn over_every_x() {
    todo!()
}
/// Based on [over_every_x()]
#[macro_export]
macro_rules! over_every_x {
    ($($t:tt)*) => {
        $crate::over_every_x()
    };
}
build_link!(overEvery, over_every);
/// **Not ported.** Returns a function OR-ing several predicates; not a Value.
pub fn over_some() {
    todo!()
}
/// Based on [over_some()]
#[macro_export]
macro_rules! over_some {
    ($($t:tt)*) => {
        $crate::over_some()
    };
}
/// **Not ported.** Returns a function OR-ing several predicates; not a Value.
pub fn over_some_x() {
    todo!()
}
/// Based on [over_some_x()]
#[macro_export]
macro_rules! over_some_x {
    ($($t:tt)*) => {
        $crate::over_some_x()
    };
}
build_link!(overSome, over_some);
/// **Not ported.** Returns a getter function for a path; not a Value.
pub fn property() {
    todo!()
}
/// Based on [property()]
#[macro_export]
macro_rules! property {
    ($($t:tt)*) => {
        $crate::property()
    };
}
/// **Not ported.** Returns a getter function for a path; not a Value.
pub fn property_x() {
    todo!()
}
/// Based on [property_x()]
#[macro_export]
macro_rules! property_x {
    ($($t:tt)*) => {
        $crate::property_x()
    };
}
/// **Not ported.** Returns a getter function bound to an object; not a Value.
pub fn property_of() {
    todo!()
}
/// Based on [property_of()]
#[macro_export]
macro_rules! property_of {
    ($($t:tt)*) => {
        $crate::property_of()
    };
}
/// **Not ported.** Returns a getter function bound to an object; not a Value.
pub fn property_of_x() {
    todo!()
}
/// Based on [property_of_x()]
#[macro_export]
macro_rules! property_of_x {
    ($($t:tt)*) => {
        $crate::property_of_x()
    };
}
build_link!(propertyOf, property_of);
mod range;
pub use range::*;
mod range_right;
pub use range_right::*;
/// **Not ported.** Creates a lodash bound to a context; not applicable to a Rust library.
pub fn run_in_context() {
    todo!()
}
/// Based on [run_in_context()]
#[macro_export]
macro_rules! run_in_context {
    ($($t:tt)*) => {
        $crate::run_in_context()
    };
}
/// **Not ported.** Creates a lodash bound to a context; not applicable to a Rust library.
pub fn run_in_context_x() {
    todo!()
}
/// Based on [run_in_context_x()]
#[macro_export]
macro_rules! run_in_context_x {
    ($($t:tt)*) => {
        $crate::run_in_context_x()
    };
}
build_link!(runInContext, run_in_context);
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
