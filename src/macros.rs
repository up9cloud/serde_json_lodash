#![macro_use]

// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

macro_rules! build_link {
    (
        $(#[doc = $doc_fn:tt])*
        $from:ident,
        $(#[doc = $doc_macro:tt])*
        $to:ident
    ) => {
        $(#[doc = $doc_fn])*
        pub use $crate::$to as $from;

        with_dollar_sign! {
            ($d:tt) => {
                $(#[doc = $doc_macro])*
                #[macro_export]
                macro_rules! $from {
                    ($d($d rest:tt)*) => {
                        $crate::$to!($d($d rest)*)
                    }
                }
            }
        }
    };
}
