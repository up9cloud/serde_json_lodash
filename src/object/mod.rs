mod assign;
pub use assign::*;
// In JSON there is no prototype chain, so the `*In` variants behave like the
// own-property variants
#[cfg(feature = "alias")]
build_link!(assign_in, assign);
#[cfg(feature = "camel")]
build_camel_link!(assignIn, assign);
#[cfg(feature = "alias")]
build_link!(extend, assign);
mod assign_with;
pub use assign_with::*;
#[cfg(feature = "alias")]
build_link!(assign_in_with, assign_with);
#[cfg(feature = "camel")]
build_camel_link!(assignInWith, assign_with);
#[cfg(feature = "alias")]
build_link!(extend_with, assign_with);
#[cfg(feature = "camel")]
build_camel_link!(extendWith, assign_with);
mod at;
pub use at::*;
/// **Not ported.** Creates an object with a given prototype; JSON has no prototype chain.
pub fn create() {
    todo!()
}
/// **Not ported.** Creates an object with a given prototype; JSON has no prototype chain.
#[macro_export]
macro_rules! create {
    ($($t:tt)*) => {
        $crate::create()
    };
}
/// **Not ported.** Creates an object with a given prototype; JSON has no prototype chain.
pub fn create_x() {
    todo!()
}
/// **Not ported.** Creates an object with a given prototype; JSON has no prototype chain.
#[macro_export]
macro_rules! create_x {
    ($($t:tt)*) => {
        $crate::create_x()
    };
}
mod defaults;
pub use defaults::*;
mod defaults_deep;
pub use defaults_deep::*;
mod find_key;
pub use find_key::*;
mod find_last_key;
pub use find_last_key::*;
mod for_own;
pub use for_own::*;
#[cfg(feature = "alias")]
build_link!(for_in, for_own);
#[cfg(feature = "camel")]
build_camel_link!(forIn, for_own);
mod for_own_right;
pub use for_own_right::*;
#[cfg(feature = "alias")]
build_link!(for_in_right, for_own_right);
#[cfg(feature = "camel")]
build_camel_link!(forInRight, for_own_right);
mod functions;
pub use functions::*;
#[cfg(feature = "alias")]
build_link!(functions_in, functions);
#[cfg(feature = "camel")]
build_camel_link!(functionsIn, functions);

mod get;
pub use get::*;

mod has;
pub use has::*;
#[cfg(feature = "alias")]
build_link!(has_in, has);
#[cfg(feature = "camel")]
build_camel_link!(hasIn, has);
mod invert;
pub use invert::*;
mod invert_by;
pub use invert_by::*;
/// **Not ported.** Invokes the method at `path`; JSON values have no methods.
pub fn invoke() {
    todo!()
}
/// **Not ported.** Invokes the method at `path`; JSON values have no methods.
#[macro_export]
macro_rules! invoke {
    ($($t:tt)*) => {
        $crate::invoke()
    };
}
/// **Not ported.** Invokes the method at `path`; JSON values have no methods.
pub fn invoke_x() {
    todo!()
}
/// **Not ported.** Invokes the method at `path`; JSON values have no methods.
#[macro_export]
macro_rules! invoke_x {
    ($($t:tt)*) => {
        $crate::invoke_x()
    };
}
mod keys;
pub use keys::*;
#[cfg(feature = "alias")]
build_link!(keys_in, keys);
#[cfg(feature = "camel")]
build_camel_link!(keysIn, keys);
mod map_keys;
pub use map_keys::*;
mod map_values;
pub use map_values::*;

mod merge;
pub use merge::*;

mod merge_with;
pub use merge_with::*;
mod omit;
pub use omit::*;
mod omit_by;
pub use omit_by::*;
mod pick;
pub use pick::*;
mod pick_by;
pub use pick_by::*;
mod result;
pub use result::*;
mod set;
pub use set::*;
/// **Not ported.** Like `set` but with a customizer for creating intermediate objects; niche, not ported.
pub fn set_with() {
    todo!()
}
/// **Not ported.** Like `set` but with a customizer for creating intermediate objects; niche, not ported.
#[macro_export]
macro_rules! set_with {
    ($($t:tt)*) => {
        $crate::set_with()
    };
}
/// **Not ported.** Like `set` but with a customizer for creating intermediate objects; niche, not ported.
pub fn set_with_x() {
    todo!()
}
/// **Not ported.** Like `set` but with a customizer for creating intermediate objects; niche, not ported.
#[macro_export]
macro_rules! set_with_x {
    ($($t:tt)*) => {
        $crate::set_with_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(setWith, set_with);
mod to_pairs;
pub use to_pairs::*;
#[cfg(feature = "alias")]
build_link!(entries, to_pairs);
#[cfg(feature = "alias")]
build_link!(to_pairs_in, to_pairs);
#[cfg(feature = "camel")]
build_camel_link!(toPairsIn, to_pairs);
#[cfg(feature = "alias")]
build_link!(entries_in, to_pairs);
#[cfg(feature = "camel")]
build_camel_link!(entriesIn, to_pairs);
mod transform;
pub use transform::*;
mod unset;
pub use unset::*;
mod update;
pub use update::*;
/// **Not ported.** Like `update` but with a customizer for creating intermediate objects; niche, not ported.
pub fn update_with() {
    todo!()
}
/// **Not ported.** Like `update` but with a customizer for creating intermediate objects; niche, not ported.
#[macro_export]
macro_rules! update_with {
    ($($t:tt)*) => {
        $crate::update_with()
    };
}
/// **Not ported.** Like `update` but with a customizer for creating intermediate objects; niche, not ported.
pub fn update_with_x() {
    todo!()
}
/// **Not ported.** Like `update` but with a customizer for creating intermediate objects; niche, not ported.
#[macro_export]
macro_rules! update_with_x {
    ($($t:tt)*) => {
        $crate::update_with_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(updateWith, update_with);
mod values;
pub use values::*;
#[cfg(feature = "alias")]
build_link!(values_in, values);
#[cfg(feature = "camel")]
build_camel_link!(valuesIn, values);
