/// **Not ported.** Returns a function invoked only after N calls; a function is not a serde_json::Value.
pub fn after() {
    todo!()
}
/// Based on [after()]
#[macro_export]
macro_rules! after {
    ($($t:tt)*) => {
        $crate::after()
    };
}
/// **Not ported.** Returns a function invoked only after N calls; a function is not a serde_json::Value.
pub fn after_x() {
    todo!()
}
/// Based on [after_x()]
#[macro_export]
macro_rules! after_x {
    ($($t:tt)*) => {
        $crate::after_x()
    };
}
/// **Not ported.** Caps a function's argument count; operates on a function, not a Value.
pub fn ary() {
    todo!()
}
/// Based on [ary()]
#[macro_export]
macro_rules! ary {
    ($($t:tt)*) => {
        $crate::ary()
    };
}
/// **Not ported.** Caps a function's argument count; operates on a function, not a Value.
pub fn ary_x() {
    todo!()
}
/// Based on [ary_x()]
#[macro_export]
macro_rules! ary_x {
    ($($t:tt)*) => {
        $crate::ary_x()
    };
}
/// **Not ported.** Returns a function invoked at most N times; not a Value.
pub fn before() {
    todo!()
}
/// Based on [before()]
#[macro_export]
macro_rules! before {
    ($($t:tt)*) => {
        $crate::before()
    };
}
/// **Not ported.** Returns a function invoked at most N times; not a Value.
pub fn before_x() {
    todo!()
}
/// Based on [before_x()]
#[macro_export]
macro_rules! before_x {
    ($($t:tt)*) => {
        $crate::before_x()
    };
}
/// **Not ported.** Binds a function to a `this`/arguments; functions are not Values.
pub fn bind() {
    todo!()
}
/// Based on [bind()]
#[macro_export]
macro_rules! bind {
    ($($t:tt)*) => {
        $crate::bind()
    };
}
/// **Not ported.** Binds a function to a `this`/arguments; functions are not Values.
pub fn bind_x() {
    todo!()
}
/// Based on [bind_x()]
#[macro_export]
macro_rules! bind_x {
    ($($t:tt)*) => {
        $crate::bind_x()
    };
}
/// **Not ported.** Binds an object method by key; JSON objects have no methods.
pub fn bind_key() {
    todo!()
}
/// Based on [bind_key()]
#[macro_export]
macro_rules! bind_key {
    ($($t:tt)*) => {
        $crate::bind_key()
    };
}
/// **Not ported.** Binds an object method by key; JSON objects have no methods.
pub fn bind_key_x() {
    todo!()
}
/// Based on [bind_key_x()]
#[macro_export]
macro_rules! bind_key_x {
    ($($t:tt)*) => {
        $crate::bind_key_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(bindKey, bind_key);
/// **Not ported.** Curries a function; operates on a function, not a Value.
pub fn curry() {
    todo!()
}
/// Based on [curry()]
#[macro_export]
macro_rules! curry {
    ($($t:tt)*) => {
        $crate::curry()
    };
}
/// **Not ported.** Curries a function; operates on a function, not a Value.
pub fn curry_x() {
    todo!()
}
/// Based on [curry_x()]
#[macro_export]
macro_rules! curry_x {
    ($($t:tt)*) => {
        $crate::curry_x()
    };
}
/// **Not ported.** Curries a function from the right; not a Value.
pub fn curry_right() {
    todo!()
}
/// Based on [curry_right()]
#[macro_export]
macro_rules! curry_right {
    ($($t:tt)*) => {
        $crate::curry_right()
    };
}
/// **Not ported.** Curries a function from the right; not a Value.
pub fn curry_right_x() {
    todo!()
}
/// Based on [curry_right_x()]
#[macro_export]
macro_rules! curry_right_x {
    ($($t:tt)*) => {
        $crate::curry_right_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(curryRight, curry_right);
/// **Not ported.** Wraps a function with debouncing; time/closures are not Values.
pub fn debounce() {
    todo!()
}
/// Based on [debounce()]
#[macro_export]
macro_rules! debounce {
    ($($t:tt)*) => {
        $crate::debounce()
    };
}
/// **Not ported.** Wraps a function with debouncing; time/closures are not Values.
pub fn debounce_x() {
    todo!()
}
/// Based on [debounce_x()]
#[macro_export]
macro_rules! debounce_x {
    ($($t:tt)*) => {
        $crate::debounce_x()
    };
}
/// **Not ported.** Defers invoking a function; there is no function to invoke.
pub fn defer() {
    todo!()
}
/// Based on [defer()]
#[macro_export]
macro_rules! defer {
    ($($t:tt)*) => {
        $crate::defer()
    };
}
/// **Not ported.** Defers invoking a function; there is no function to invoke.
pub fn defer_x() {
    todo!()
}
/// Based on [defer_x()]
#[macro_export]
macro_rules! defer_x {
    ($($t:tt)*) => {
        $crate::defer_x()
    };
}
/// **Not ported.** Invokes a function after a delay; not a Value.
pub fn delay() {
    todo!()
}
/// Based on [delay()]
#[macro_export]
macro_rules! delay {
    ($($t:tt)*) => {
        $crate::delay()
    };
}
/// **Not ported.** Invokes a function after a delay; not a Value.
pub fn delay_x() {
    todo!()
}
/// Based on [delay_x()]
#[macro_export]
macro_rules! delay_x {
    ($($t:tt)*) => {
        $crate::delay_x()
    };
}
/// **Not ported.** Returns a function with reversed arguments; not a Value.
pub fn flip() {
    todo!()
}
/// Based on [flip()]
#[macro_export]
macro_rules! flip {
    ($($t:tt)*) => {
        $crate::flip()
    };
}
/// **Not ported.** Returns a function with reversed arguments; not a Value.
pub fn flip_x() {
    todo!()
}
/// Based on [flip_x()]
#[macro_export]
macro_rules! flip_x {
    ($($t:tt)*) => {
        $crate::flip_x()
    };
}
/// **Not ported.** Memoizes a function; operates on a function, not a Value.
pub fn memoize() {
    todo!()
}
/// Based on [memoize()]
#[macro_export]
macro_rules! memoize {
    ($($t:tt)*) => {
        $crate::memoize()
    };
}
/// **Not ported.** Memoizes a function; operates on a function, not a Value.
pub fn memoize_x() {
    todo!()
}
/// Based on [memoize_x()]
#[macro_export]
macro_rules! memoize_x {
    ($($t:tt)*) => {
        $crate::memoize_x()
    };
}
/// **Not ported.** Returns a negated predicate function; not a Value.
pub fn negate() {
    todo!()
}
/// Based on [negate()]
#[macro_export]
macro_rules! negate {
    ($($t:tt)*) => {
        $crate::negate()
    };
}
/// **Not ported.** Returns a negated predicate function; not a Value.
pub fn negate_x() {
    todo!()
}
/// Based on [negate_x()]
#[macro_export]
macro_rules! negate_x {
    ($($t:tt)*) => {
        $crate::negate_x()
    };
}
/// **Not ported.** Returns a function callable once; not a Value.
pub fn once() {
    todo!()
}
/// Based on [once()]
#[macro_export]
macro_rules! once {
    ($($t:tt)*) => {
        $crate::once()
    };
}
/// **Not ported.** Returns a function callable once; not a Value.
pub fn once_x() {
    todo!()
}
/// Based on [once_x()]
#[macro_export]
macro_rules! once_x {
    ($($t:tt)*) => {
        $crate::once_x()
    };
}
/// **Not ported.** Transforms a function's arguments; not a Value.
pub fn over_args() {
    todo!()
}
/// Based on [over_args()]
#[macro_export]
macro_rules! over_args {
    ($($t:tt)*) => {
        $crate::over_args()
    };
}
/// **Not ported.** Transforms a function's arguments; not a Value.
pub fn over_args_x() {
    todo!()
}
/// Based on [over_args_x()]
#[macro_export]
macro_rules! over_args_x {
    ($($t:tt)*) => {
        $crate::over_args_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(overArgs, over_args);
/// **Not ported.** Partially applies a function; not a Value.
pub fn partial() {
    todo!()
}
/// Based on [partial()]
#[macro_export]
macro_rules! partial {
    ($($t:tt)*) => {
        $crate::partial()
    };
}
/// **Not ported.** Partially applies a function; not a Value.
pub fn partial_x() {
    todo!()
}
/// Based on [partial_x()]
#[macro_export]
macro_rules! partial_x {
    ($($t:tt)*) => {
        $crate::partial_x()
    };
}
/// **Not ported.** Partially applies from the right; not a Value.
pub fn partial_right() {
    todo!()
}
/// Based on [partial_right()]
#[macro_export]
macro_rules! partial_right {
    ($($t:tt)*) => {
        $crate::partial_right()
    };
}
/// **Not ported.** Partially applies from the right; not a Value.
pub fn partial_right_x() {
    todo!()
}
/// Based on [partial_right_x()]
#[macro_export]
macro_rules! partial_right_x {
    ($($t:tt)*) => {
        $crate::partial_right_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(partialRight, partial_right);
/// **Not ported.** Reorders a function's arguments; not a Value.
pub fn rearg() {
    todo!()
}
/// Based on [rearg()]
#[macro_export]
macro_rules! rearg {
    ($($t:tt)*) => {
        $crate::rearg()
    };
}
/// **Not ported.** Reorders a function's arguments; not a Value.
pub fn rearg_x() {
    todo!()
}
/// Based on [rearg_x()]
#[macro_export]
macro_rules! rearg_x {
    ($($t:tt)*) => {
        $crate::rearg_x()
    };
}
/// **Not ported.** Turns trailing arguments into an array parameter; not a Value.
pub fn rest() {
    todo!()
}
/// Based on [rest()]
#[macro_export]
macro_rules! rest {
    ($($t:tt)*) => {
        $crate::rest()
    };
}
/// **Not ported.** Turns trailing arguments into an array parameter; not a Value.
pub fn rest_x() {
    todo!()
}
/// Based on [rest_x()]
#[macro_export]
macro_rules! rest_x {
    ($($t:tt)*) => {
        $crate::rest_x()
    };
}
/// **Not ported.** Spreads an array into a function's arguments; not a Value.
pub fn spread() {
    todo!()
}
/// Based on [spread()]
#[macro_export]
macro_rules! spread {
    ($($t:tt)*) => {
        $crate::spread()
    };
}
/// **Not ported.** Spreads an array into a function's arguments; not a Value.
pub fn spread_x() {
    todo!()
}
/// Based on [spread_x()]
#[macro_export]
macro_rules! spread_x {
    ($($t:tt)*) => {
        $crate::spread_x()
    };
}
/// **Not ported.** Wraps a function with throttling; not a Value.
pub fn throttle() {
    todo!()
}
/// Based on [throttle()]
#[macro_export]
macro_rules! throttle {
    ($($t:tt)*) => {
        $crate::throttle()
    };
}
/// **Not ported.** Wraps a function with throttling; not a Value.
pub fn throttle_x() {
    todo!()
}
/// Based on [throttle_x()]
#[macro_export]
macro_rules! throttle_x {
    ($($t:tt)*) => {
        $crate::throttle_x()
    };
}
/// **Not ported.** Caps a function to one argument; not a Value.
pub fn unary() {
    todo!()
}
/// Based on [unary()]
#[macro_export]
macro_rules! unary {
    ($($t:tt)*) => {
        $crate::unary()
    };
}
/// **Not ported.** Caps a function to one argument; not a Value.
pub fn unary_x() {
    todo!()
}
/// Based on [unary_x()]
#[macro_export]
macro_rules! unary_x {
    ($($t:tt)*) => {
        $crate::unary_x()
    };
}
/// **Not ported.** Wraps a value in a function; the result is a function, not a Value.
pub fn wrap() {
    todo!()
}
/// Based on [wrap()]
#[macro_export]
macro_rules! wrap {
    ($($t:tt)*) => {
        $crate::wrap()
    };
}
/// **Not ported.** Wraps a value in a function; the result is a function, not a Value.
pub fn wrap_x() {
    todo!()
}
/// Based on [wrap_x()]
#[macro_export]
macro_rules! wrap_x {
    ($($t:tt)*) => {
        $crate::wrap_x()
    };
}
