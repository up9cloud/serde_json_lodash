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
        $(#[doc = $doc:tt])*
        $from:ident,
        $to:ident
    ) => {
        ///
        pub use $crate::$to as $from;

        with_dollar_sign! {
            ($d:tt) => {
                $(#[doc = $doc])*
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
