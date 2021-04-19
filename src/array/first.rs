build_link!(
    /// Description can be found in [lodash first](https://lodash.com/docs/#first)
    ///
    /// More examples:
    ///
    /// ```rust
    /// # #[macro_use] extern crate serde_json_lodash;
    /// # use serde_json::json;
    /// assert_eq!(first!(), json!(null));
    /// assert_eq!(first!(json!("東京")), json!("東"));
    /// ```
    first,
    head
);
