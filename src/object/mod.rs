mod assign;
pub use assign::*;
// In JSON there is no prototype chain, so the `*In` variants behave like the
// own-property variants
#[doc(hidden)]
pub use assign as assign_in;
#[doc(hidden)]
pub use assign as assignIn;
#[doc(hidden)]
pub use assign as extend;
mod assign_with;
pub use assign_with::*;
#[doc(hidden)]
pub use assign_with as assign_in_with;
#[doc(hidden)]
pub use assign_with as assignInWith;
#[doc(hidden)]
pub use assign_with as extend_with;
#[doc(hidden)]
pub use assign_with as extendWith;
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
#[doc(hidden)]
pub use for_own as for_in;
#[doc(hidden)]
pub use for_own as forIn;
mod for_own_right;
pub use for_own_right::*;
#[doc(hidden)]
pub use for_own_right as for_in_right;
#[doc(hidden)]
pub use for_own_right as forInRight;
mod functions;
pub use functions::*;
#[doc(hidden)]
pub use functions as functions_in;
#[doc(hidden)]
pub use functions as functionsIn;

mod get;
pub use get::*;

mod has;
pub use has::*;
#[doc(hidden)]
pub use has as has_in;
#[doc(hidden)]
pub use has as hasIn;
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
#[doc(hidden)]
pub use keys as keys_in;
#[doc(hidden)]
pub use keys as keysIn;
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
#[doc(hidden)]
pub use set_with as setWith;
mod to_pairs;
pub use to_pairs::*;
#[doc(hidden)]
pub use to_pairs as entries;
#[doc(hidden)]
pub use to_pairs as to_pairs_in;
#[doc(hidden)]
pub use to_pairs as toPairsIn;
#[doc(hidden)]
pub use to_pairs as entries_in;
#[doc(hidden)]
pub use to_pairs as entriesIn;
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
#[doc(hidden)]
pub use update_with as updateWith;
mod values;
pub use values::*;
#[doc(hidden)]
pub use values as values_in;
#[doc(hidden)]
pub use values as valuesIn;
