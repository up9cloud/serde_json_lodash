mod camel_case;
pub use camel_case::*;
mod capitalize;
pub use capitalize::*;
mod deburr;
pub use deburr::*;
mod ends_with;
pub use ends_with::*;
mod escape;
pub use escape::*;
mod escape_reg_exp;
pub use escape_reg_exp::*;
mod kebab_case;
pub use kebab_case::*;
mod lower_case;
pub use lower_case::*;
mod lower_first;
pub use lower_first::*;
mod pad;
pub use pad::*;
mod pad_end;
pub use pad_end::*;
mod pad_start;
pub use pad_start::*;
mod parse_int;
pub use parse_int::*;
mod repeat;
pub use repeat::*;
mod replace;
pub use replace::*;
mod snake_case;
pub use snake_case::*;
mod split;
pub use split::*;
mod start_case;
pub use start_case::*;
mod starts_with;
pub use starts_with::*;
/// **Not ported.** Compiles a string into a render function; requires a template engine, out of scope.
pub fn template() {
    todo!()
}
/// Based on [template()]
#[macro_export]
macro_rules! template {
    ($($t:tt)*) => {
        $crate::template()
    };
}
/// **Not ported.** Compiles a string into a render function; requires a template engine, out of scope.
pub fn template_x() {
    todo!()
}
/// Based on [template_x()]
#[macro_export]
macro_rules! template_x {
    ($($t:tt)*) => {
        $crate::template_x()
    };
}
mod to_lower;
pub use to_lower::*;
mod to_upper;
pub use to_upper::*;
mod trim;
pub use trim::*;
mod trim_end;
pub use trim_end::*;
mod trim_start;
pub use trim_start::*;
mod truncate;
pub use truncate::*;
mod unescape;
pub use unescape::*;
mod upper_case;
pub use upper_case::*;
mod upper_first;
pub use upper_first::*;
mod words;
pub use words::*;
