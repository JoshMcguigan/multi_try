pub trait MultiTry<RT, ERR> {
    type Output;

    fn and_try(self, other: Result<RT, ERR>) -> Self::Output;
}

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
