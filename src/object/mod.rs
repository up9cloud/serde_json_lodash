mod assign;
pub use assign::*;
// In JSON there is no prototype chain, so the `*In` variants behave like the
// own-property variants
build_link!(assign_in, assign);
build_link!(assignIn, assign);
build_link!(extend, assign);
mod assign_with;
pub use assign_with::*;
build_link!(assign_in_with, assign_with);
build_link!(assignInWith, assign_with);
build_link!(extend_with, assign_with);
build_link!(extendWith, assign_with);
mod at;
pub use at::*;
/// **Not ported.** Creates an object with a given prototype; JSON has no prototype chain.
pub fn create() {
    todo!()
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
build_link!(for_in, for_own);
build_link!(forIn, for_own);
mod for_own_right;
pub use for_own_right::*;
build_link!(for_in_right, for_own_right);
build_link!(forInRight, for_own_right);
mod functions;
pub use functions::*;
build_link!(functions_in, functions);
build_link!(functionsIn, functions);

mod get;
pub use get::*;

mod has;
pub use has::*;
build_link!(has_in, has);
build_link!(hasIn, has);
mod invert;
pub use invert::*;
mod invert_by;
pub use invert_by::*;
/// **Not ported.** Invokes the method at `path`; JSON values have no methods.
pub fn invoke() {
    todo!()
}
mod keys;
pub use keys::*;
build_link!(keys_in, keys);
build_link!(keysIn, keys);
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
build_link_fn!(setWith, set_with);
mod to_pairs;
pub use to_pairs::*;
build_link!(entries, to_pairs);
build_link!(to_pairs_in, to_pairs);
build_link!(toPairsIn, to_pairs);
build_link!(entries_in, to_pairs);
build_link!(entriesIn, to_pairs);
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
build_link_fn!(updateWith, update_with);
mod values;
pub use values::*;
build_link!(values_in, values);
build_link!(valuesIn, values);
