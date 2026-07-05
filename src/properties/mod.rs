/// See lodash [VERSION](https://lodash.com/docs/#VERSION)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// The other lodash "Properties" entry, `_.templateSettings` (with its
// `.escape` / `.evaluate` / `.interpolate` / `.imports` / `.variable`
// sub-fields), configures the string template engine. That engine is not
// ported (see `template`), and the dotted names are not valid Rust
// identifiers, so nothing is exposed for it here.
