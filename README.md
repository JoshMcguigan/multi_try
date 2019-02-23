# multi_try [![crates.io badge](https://img.shields.io/crates/v/multi_try.svg)](https://crates.io/crates/multi_try) [![Build Status](https://travis-ci.org/JoshMcguigan/multi_try.svg?branch=master)](https://travis-ci.org/JoshMcguigan/multi_try)

This crate allows combining multiple `Result` types, and returning either a tuple containing all of their results, or a `Vec` of any errors which occurred. It is useful when you want to provide an error message for all errors rather than simply returning the first error.

Generics are used to support `Result<T, E>` for any types of `T` and `E`. The `ok` types of the combined results are NOT required to be the same, but all of the `error` types must be the same. 

## Example

```rust
struct A {
    b: Result<i32, MyErr>,
    c: Result<i64, MyErr>,
    d: Result<f32, MyErr>,
}

struct ValidatedA {
    b: i32,
    c: i64,
    d: f32,
}

enum MyErr {
    FailedB,
    FailedC,
    FailedD,
}

fn validate(a: A) -> Result<ValidatedA, Vec<MyErr>> {
    let (b, c, d) =
        multi_try::and(
            a.b,
            a.c
        )
        .and(a.d)
        .into_result()?;

    Ok(ValidatedA { b, c, d })
}
```

Check the `tests` directory for additional examples.

#### Enabling nightly features

Enabling the `nightly` feature for `multi_try` allows removing the call to `into_result` before using the `?` operator. 

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
