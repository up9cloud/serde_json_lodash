# Contributing

## Design priorities

When goals conflict, they rank in this order — never trade a higher one for a lower one:

1. **lodash API fidelity** — names, signatures, argument order, and behavior follow the lodash docs as closely as Rust allows.
2. **Ergonomics** — easy to call: `Into<Value>` inputs (primitives or `json!` values), macros for lodash-style optional/variadic args, `impl Fn` callbacks that accept capturing closures.
3. **Performance** — optimize only within the bounds of 1 and 2.

## Dev memo

Dev commands are cargo aliases defined in [`.cargo/config.toml`](.cargo/config.toml):

```bash
cargo t                        # run all tests (= cargo test --features lazy_static)
cargo t --doc set              # run one function's doc tests once
cargo dev                      # watch + test everything (needs cargo-watch)

cargo fmt --all && cargo lint  # format + clippy, same gates CI enforces
cargo doc --open               # preview docs
cargo bench                    # criterion benches (curated, see benches/criterion.rs)
```

Tests live in the doc comments: the `Examples:` block mirrors the lodash
documentation for that function, and `Additional cases:` covers everything
else worth showing (edge cases, empty/optional args, type coercion, JSON
specifics).

## Releasing a new version

- Commit all your changes first (a clean working tree keeps the tag accurate).
- Update version in Cargo.toml
- Add git tag (without v prefix), e.g. `0.0.1`, NOT v0.0.1
- Push the commit and the tag — pushing the tag triggers CI to publish to crates.io:

   ```bash
   git push && git push --tags
   ```

## Checking lodash's real behavior

```console
# node
Welcome to Node.js v24.1.0.
Type ".help" for more information.
> const {default: l} = await import('lodash')
undefined
> l.capitalize('FRED')
'Fred'
```
