//! lodash's `_(value)` "Seq" chaining wrapper.
//!
//! Chaining builds a lazy wrapper around a value and defers a pipeline of
//! operations; that wrapper has no equivalent when every function already
//! takes and returns an owned [`Value`](crate::lib::Value), so the Seq API is
//! not ported. The entry points that have valid Rust names are listed below as
//! not-ported stubs; the rest (`_` itself and the `prototype.*` methods such
//! as `prototype.value`, `prototype.at`, `prototype[Symbol.iterator]`, …)
//! cannot be expressed as Rust identifiers and are omitted.

/// **Not ported.** Wraps a value to enable explicit method chaining; this
/// crate has no chaining wrapper (call the functions directly instead).
pub fn chain() {
    todo!()
}
/// Based on [chain()]
#[macro_export]
macro_rules! chain {
    ($($t:tt)*) => {
        $crate::chain()
    };
}
/// **Not ported.** Wraps a value to enable explicit method chaining; this
/// crate has no chaining wrapper (call the functions directly instead).
pub fn chain_x() {
    todo!()
}
/// Based on [chain_x()]
#[macro_export]
macro_rules! chain_x {
    ($($t:tt)*) => {
        $crate::chain_x()
    };
}
/// **Not ported.** Invokes an interceptor with the value then returns it; part
/// of the unsupported chaining wrapper.
pub fn tap() {
    todo!()
}
/// Based on [tap()]
#[macro_export]
macro_rules! tap {
    ($($t:tt)*) => {
        $crate::tap()
    };
}
/// **Not ported.** Invokes an interceptor with the value then returns it; part
/// of the unsupported chaining wrapper.
pub fn tap_x() {
    todo!()
}
/// Based on [tap_x()]
#[macro_export]
macro_rules! tap_x {
    ($($t:tt)*) => {
        $crate::tap_x()
    };
}
/// **Not ported.** Passes the value through an interceptor and returns its
/// result; part of the unsupported chaining wrapper.
pub fn thru() {
    todo!()
}
/// Based on [thru()]
#[macro_export]
macro_rules! thru {
    ($($t:tt)*) => {
        $crate::thru()
    };
}
/// **Not ported.** Passes the value through an interceptor and returns its
/// result; part of the unsupported chaining wrapper.
pub fn thru_x() {
    todo!()
}
/// Based on [thru_x()]
#[macro_export]
macro_rules! thru_x {
    ($($t:tt)*) => {
        $crate::thru_x()
    };
}
