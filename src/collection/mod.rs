mod collect;
mod count_by;
pub use count_by::*;
mod each;
pub use each::*;
#[cfg(feature = "alias")]
build_link!(for_each, each);
mod each_right;
pub use each_right::*;
#[cfg(feature = "alias")]
build_link!(for_each_right, each_right);
mod every;
pub use every::*;
mod filter;
pub use filter::*;
mod find;
pub use find::*;
mod find_last;
pub use find_last::*;
mod flat_map;
pub use flat_map::*;
mod flat_map_deep;
pub use flat_map_deep::*;
mod flat_map_depth;
pub use flat_map_depth::*;
mod group_by;
pub use group_by::*;
mod includes;
pub use includes::*;
/// **Not ported.** Invokes a named method on each element; JSON values have no methods.
pub fn invoke_map() {
    todo!()
}
/// **Not ported.** Invokes a named method on each element; JSON values have no methods.
#[macro_export]
macro_rules! invoke_map {
    ($($t:tt)*) => {
        $crate::invoke_map()
    };
}
/// **Not ported.** Invokes a named method on each element; JSON values have no methods.
pub fn invoke_map_x() {
    todo!()
}
/// **Not ported.** Invokes a named method on each element; JSON values have no methods.
#[macro_export]
macro_rules! invoke_map_x {
    ($($t:tt)*) => {
        $crate::invoke_map_x()
    };
}
#[cfg(feature = "camel")]
build_camel_link!(invokeMap, invoke_map);
mod key_by;
pub use key_by::*;
mod map;
pub use map::*;
mod order_by;
pub use order_by::*;
mod partition;
pub use partition::*;
mod reduce;
pub use reduce::*;
mod reduce_right;
pub use reduce_right::*;
mod reject;
pub use reject::*;
mod sample;
pub use sample::*;
mod sample_size;
pub use sample_size::*;
mod shuffle;
pub use shuffle::*;
mod size;
pub use size::*;
mod some;
pub use some::*;
mod sort_by;
pub use sort_by::*;
