# modificable
Rust setters generator via proc macro and provide `Modifications` trait.

## Public Features
- `modifications_utc`: Enables the `ModificationsMetadata` struct and the `Modifications` trait implementation for `chrono::Utc`.
- `modifications_local`: Enables the `ModificationsMetadata` struct and the `Modifications` trait implementation for `chrono::Local`.

Don't use both features at the same time, because they are mutually exclusive. Please, don't enable any other feature, they are only for internal use.



## Tests
Use the next command to run tests, otherwise you can't really test anything:
```bash
cargo test --features testing
```