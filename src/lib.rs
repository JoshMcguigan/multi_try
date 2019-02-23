//! This crate allows you to combine multiple `Result` types and return either a
//! tuple containing all of their results, or a `Vec` of any errors which occurred.
//! It is useful when you want to provide an error message for all errors rather
//! than simply returning the first error.
//!
//! Generics are used to support `Result<T, E>` for any types of `T` and `E`. The
//! `Ok` types of the combined results are NOT required to be the same, but all of
//! the `Err` types must be the same.
//!
//! See the documentation for the [`MultiTry` trait] for more information and an example.
//!
//! [`MultiTry` trait]: trait.MultiTry.html

/// Exposes the `and_try` method for combining multiple `Result` types.
///
/// This is an extension trait designed to add functionality to the `Result` type. That
/// means that to use this trait's methods, you must have it in scope:
///
/// ```no_run
/// use multi_try::MultiTry;
/// ```
///
/// The `Ok` variant of each combined `Result` can have any type. Each value will be combined into
/// a tuple. We support up to 27 items, though we never expect anyone to need anything close to
/// that. The `Err` variant of each `Result` must have the same type. Each error that occurs will
/// be combined into a single `Vec`.
///
/// The idea is that you can try many different operations, collect any errors that occurred, and
/// then only proceed if every operation was a success. The example below demonstrates that:
///
/// ```no_run
/// use multi_try::MultiTry;
///
/// struct A {
///     b: Result<i32, MyErr>,
///     c: Result<i64, MyErr>,
///     d: Result<f32, MyErr>,
/// }
///
/// struct ValidatedA {
///     b: i32,
///     c: i64,
///     d: f32,
/// }
///
/// enum MyErr {
///     FailedB,
///     FailedC,
///     FailedD,
/// }
///
/// fn validate(a: A) -> Result<ValidatedA, Vec<MyErr>> {
///     // Only continue beyond this point if all the `Result` values were `Ok`
///     let (b, c, d) = a.b.and_try(a.c).and_try(a.d)?;
///
///     Ok(ValidatedA { b, c, d })
/// }
/// ```
pub trait MultiTry<RT, ERR> {
    /// The output of the `and_try` operation
    type Output;

    /// Returns the current `Result` combined with the given `Result`. If both are `Ok`, this will
    /// return a new tuple that combines the results. If either have failed, this will return a
    /// vector containing each error that occurred.
    ///
    /// ```
    /// use multi_try::MultiTry;
    /// # // These functions are used to get around type inference issues
    /// # fn Ok<T>(value: T) -> Result<T, &'static str> { Result::Ok(value) }
    /// # fn Err<E>(err: E) -> Result<i32, E> { Result::Err(err) }
    ///
    /// # fn main() -> Result<(), Vec<&'static str>> {
    /// // Combines two results so we get a 2-tuple
    /// assert_eq!(Ok(1).and_try(Ok("abc"))?, (1, "abc"));
    /// // Combines two results with another result so we get a 3-tuple
    /// assert_eq!(Ok(1).and_try(Ok("abc")).and_try(Ok(37.4))?, (1, "abc", 37.4));
    /// // Even if one succeeds, we only return Ok() if they both do
    /// assert_eq!(Err("bad!").and_try(Ok(32)).unwrap_err(), vec!["bad!"]);
    /// assert_eq!(Ok(1).and_try(Err("very bad!")).unwrap_err(), vec!["very bad!"]);
    /// // If both fail, we return both errors
    /// assert_eq!(Err("bad!").and_try(Err("very bad!")).unwrap_err(), vec!["bad!", "very bad!"]);
    /// # Result::Ok(()) }
    /// ```
    fn and_try(self, other: Result<RT, ERR>) -> Self::Output;
}

// Allows you to combine Result<T, ERR> with Result<RT, ERR> to get Result<(T, RT), Vec<ERR>>
impl<T, RT, ERR> MultiTry<RT, ERR> for Result<T, ERR> {
    type Output = Result<(T, RT), Vec<ERR>>;

    fn and_try(self, other: Result<RT, ERR>) -> Self::Output {
        match (self, other) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(_), Err(eb)) => Err(vec![eb]),
            (Err(ea), Ok(_)) => Err(vec![ea]),
            (Err(ea), Err(eb)) => Err(vec![ea, eb]),
        }
    }
}

macro_rules! impl_multi_try {
    ($($typ:ident),+) => {
        impl<RT, ERR, $($typ),+> MultiTry<RT, ERR> for Result<($($typ),+,), Vec<ERR>> {
            type Output = Result<($($typ),*, RT), Vec<ERR>>;

            fn and_try(self, other: Result<RT, ERR>) -> Self::Output {
                #[allow(non_snake_case)] // reusing the type parameter identifiers as variables
                match (self, other) {
                    (Ok(($($typ),+,)), Ok(rt)) => Ok(($($typ),+, rt)),
                    (Ok(_), Err(e)) => Err(vec![e]),
                    (Err(errs), Ok(_)) => Err(errs),
                    (Err(mut errs), Err(e)) => Err({
                        errs.push(e);
                        errs
                    }),
                }
            }
        }
    };
}

impl_multi_try!(A);
impl_multi_try!(A, B);
impl_multi_try!(A, B, C);
impl_multi_try!(A, B, C, D);
impl_multi_try!(A, B, C, D, E);
impl_multi_try!(A, B, C, D, E, F);
impl_multi_try!(A, B, C, D, E, F, G);
impl_multi_try!(A, B, C, D, E, F, G, H);
impl_multi_try!(A, B, C, D, E, F, G, H, I);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_multi_try!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
