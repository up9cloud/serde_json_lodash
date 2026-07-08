#![macro_use]

// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

/// Generates the **Not provided.** `_x` stub pair (`fn $name_x` + `$name_x!`)
/// for a function whose result has no primitive form. The default wording
/// covers composite/runtime-dynamic `Value` results; pass a `$reason` string
/// (a complete sentence following "**Not provided.** ") for anything else,
/// e.g. the closure-returning combinators. Both names are passed explicitly
/// because deriving `_x` would need `paste`, which is an optional dependency.
macro_rules! build_not_provided_x {
    ($name:ident, $name_x:ident) => {
        build_not_provided_x!(
            $name,
            $name_x,
            ::core::concat!(
                "The result is a composite or runtime-dynamic `Value` with no single primitive to downgrade to; use [",
                ::core::stringify!($name),
                "!](crate::",
                ::core::stringify!($name),
                "!) and read the returned `Value`."
            )
        );
    };
    ($name:ident, $name_x:ident, $reason:expr) => {
        #[doc = ::core::concat!("**Not provided.** ", $reason)]
        #[doc = ""]
        #[doc = ::core::concat!("Macro form: [", ::core::stringify!($name_x), "!](crate::", ::core::stringify!($name_x), "!)")]
        pub fn $name_x() {
            todo!()
        }

        with_dollar_sign! {
            ($d:tt) => {
                #[doc = ::core::concat!("**Not provided.** ", $reason)]
                #[doc = ""]
                #[doc = ::core::concat!("Fn form: [", ::core::stringify!($name_x), "()]")]
                #[macro_export]
                macro_rules! $name_x {
                    ($d($d t:tt)*) => {
                        $crate::$name_x()
                    };
                }
            }
        }
    };
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
            // via __fn so only the fn is imported; see the module's comment
            #[doc(hidden)]
            pub use $crate::__fn::[<$to _x>] as [<$from _x>];
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
            // via __fn so only the fn is imported; see the module's comment
            #[doc(hidden)]
            pub use $crate::__fn::[<$to _x>] as [<$from X>];
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
