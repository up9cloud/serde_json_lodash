use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref COUNT: Mutex<usize> = Mutex::new(0);
}
///
pub fn unique_id(prefix: &str) -> String {
    let mut c = COUNT.lock().unwrap();
    *c += 1;
    format!("{}{}", prefix, c)
}
/// Description can be found in [lodash uniqueId](https://lodash.com/docs/#uniqueId)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   unique_id!("contact_"),
///   json!("contact_1")
/// );
/// assert_eq!(
///   unique_id!(),
///   json!("2")
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// ```
#[macro_export]
macro_rules! unique_id {
    () => {
        $crate::unique_id("")
    };
    ($a:expr $(,)*) => {
        $crate::unique_id($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::unique_id($a)
    };
}
