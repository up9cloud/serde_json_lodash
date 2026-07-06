#![macro_use]

// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
#[cfg(feature = "alias")]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

// Aliasing one lodash name to another (a name lodash itself aliases, e.g.
// `first` -> `head`, `entries` -> `toPairs`, or a camelCase spelling like
// `dropRight` -> `drop_right`) all go through this single mechanism.
//
// An alias mirrors exactly the callable forms the target exposes:
// - `build_link!` when the target has both a `fn` and a `macro` -> the alias
//   gets both.
// - `build_link_fn!` when the target only has a `fn` (e.g. a not-ported
//   `todo!()` stub, which has no macro to forward to) -> the alias gets only a
//   `fn`.
//
// A target that also has a `_x` primitive-output helper (e.g. `capitalize`
// has `capitalize_x`) is aliased by calling `build_link!` once per variant,
// for example:
//
// ```ignore
// build_links![
//     abc      => capitalize
//     abc_x    => capitalize_x
// ];
// ```
//
// Everything an alias generates is `#[doc(hidden)]`, so aliases never show up
// as a `Re-exports: pub use head as first;` entry in rustdoc; only the
// canonical name is documented, exactly like the camelCase aliases.

/// Alias `$from` to `$to`, re-exporting the whole family: the `fn`s `$from` /
/// `$from_x` and the macros `$from!` / `$from_x!`, each forwarding to the `$to`
/// equivalent. Every function now has all four forms, so a single `build_link!`
/// covers an alias completely.
#[cfg(feature = "alias")]
macro_rules! build_link {
    ($from:ident, $to:ident) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            pub use $crate::$to as $from;
            #[doc(hidden)]
            pub use $crate::[<$to _x>] as [<$from _x>];
        }

        with_dollar_sign! {
            ($d:tt) => {
                $crate::paste::paste! {
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! $from {
                        ($d($d rest:tt)*) => {
                            $crate::$to!($d($d rest)*)
                        }
                    }
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! [<$from _x>] {
                        ($d($d rest:tt)*) => {
                            $crate::[<$to _x>]!($d($d rest)*)
                        }
                    }
                }
            }
        }
    };
}

/// Alias a camelCase spelling `$from` to snake_case `$to`, covering the whole
/// family: `$from`/`$fromX` fns and `$from!`/`$fromX!` macros, forwarding to
/// `$to`/`$to_x` respectively. Like [build_link!] but the `_x` form of the
/// camelCase name is spelled with an `X` suffix (e.g. `isEmptyX` aliases
/// `is_empty_x`). Used both by `build_camel_links!` and by the camelCase
/// aliases declared in the category `mod.rs` files.
#[cfg(feature = "camel")]
macro_rules! build_camel_link {
    ($from:ident, $to:ident) => {
        $crate::paste::paste! {
            #[doc(hidden)]
            pub use $crate::$to as $from;
            #[doc(hidden)]
            pub use $crate::[<$to _x>] as [<$from X>];
        }

        with_dollar_sign! {
            ($d:tt) => {
                $crate::paste::paste! {
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! $from {
                        ($d($d rest:tt)*) => {
                            $crate::$to!($d($d rest)*)
                        }
                    }
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! [<$from X>] {
                        ($d($d rest:tt)*) => {
                            $crate::[<$to _x>]!($d($d rest)*)
                        }
                    }
                }
            }
        }
    };
}
