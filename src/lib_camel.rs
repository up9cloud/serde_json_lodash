macro_rules! build_camel_case {
    (
        $(#[doc = $doc:tt])*
        $id:ident
    ) => {
        paste::paste! {
            with_dollar_sign! {
                ($d:tt) => {
                    #[doc(hidden)]
                    pub use $id as [<$id:camel>];

                    #[doc(hidden)]
                    $(#[doc = $doc])*
                    #[macro_export]
                    macro_rules! [<$id:camel>] {
                        ($d($d rest:tt)*) => {
                            $crate::$id!($d($d rest)*)
                        }
                    }
                }
            }
        }
    }
}

pub use crate::lib_snake::*;

build_camel_case!(
    /// More examples:
    ///
    /// ```rust
    /// # #[macro_use] extern crate serde_json_lodash;
    /// # use serde_json::json;
    /// assert_eq!(dropRight!(), json!([]));
    /// ```
    drop_right
);

macro_rules! build_multi {
    [
        $($id:ident)+
    ] => {
        $(
            build_camel_case!(
                ///
                $id
            );
        )+
    }
}

build_multi![
    find_index
    find_last_index
    flatten_deep
    flatten_depth
    from_pairs
    index_of
    last_index_of
    pull_all
    pull_all_by

    to_string

    to_lower
];

#[cfg(feature = "lazy_static")]
build_multi![unique_id];
