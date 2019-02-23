#![cfg_attr(feature = "nightly", feature(try_trait))]

pub struct Combined2<A, B, ERR> (Result<(A, B), Vec<ERR>>);
pub struct Combined3<A, B, C, ERR> (Result<(A, B, C), Vec<ERR>>);
pub struct Combined4<A, B, C, D, ERR> (Result<(A, B, C, D), Vec<ERR>>);

pub fn and<A, B, ERR>(a: Result<A, ERR>, b: Result<B, ERR>) -> Combined2<A, B, ERR> {
    match (a, b) {
        (Ok(a), Ok(b)) => Combined2(Ok((a, b))),
        (Ok(_a), Err(b)) => Combined2(Err(vec![b])),
        (Err(a), Ok(_b)) => Combined2(Err(vec![a])),
        (Err(a), Err(b)) => Combined2(Err(vec![a, b])),
    }
}

impl<A, B, ERR> Combined2<A, B, ERR> {
    pub fn and<C>(self, other: Result<C, ERR>) -> Combined3<A, B, C, ERR> {
        match (self.0, other) {
            (Ok(vals), Ok(new_val)) => {
                Combined3(Ok((vals.0, vals.1, new_val)))
            },
            (Err(errors), Ok(_)) => {
                Combined3(Err(errors))
            },
            (Ok(_vals), Err(e)) => {
                Combined3(Err(vec![e]))
            },
            (Err(mut errors), Err(e)) => {
                errors.push(e);
                Combined3(Err(errors))
            },
        }
    }

    pub fn into_result(self) -> Result<(A, B), Vec<ERR>> {
        self.0
    }
}

impl<A, B, C, ERR> Combined3<A, B, C, ERR> {
    pub fn and<D>(self, other: Result<D, ERR>) -> Combined4<A, B, C, D, ERR> {
        match (self.0, other) {
            (Ok(vals), Ok(new_val)) => {
                Combined4(Ok((vals.0, vals.1, vals.2, new_val)))
            },
            (Err(errors), Ok(_)) => {
                Combined4(Err(errors))
            },
            (Ok(_vals), Err(e)) => {
                Combined4(Err(vec![e]))
            },
            (Err(mut errors), Err(e)) => {
                errors.push(e);
                Combined4(Err(errors))
            },
        }
    }

    pub fn into_result(self) -> Result<(A, B, C), Vec<ERR>> {
        self.0
    }
}

impl<A, B, C, D, ERR> Combined4<A, B, C, D, ERR> {
    pub fn into_result(self) -> Result<(A, B, C, D), Vec<ERR>> {
        self.0
    }
}

#[cfg(feature = "nightly")]
mod try_impl {
    use super::{Combined2, Combined3, Combined4};
    use std::ops::Try;

    impl<A, B, ERR> Try for Combined2<A, B, ERR> {
        type Ok = (A, B);
        type Error = Vec<ERR>;

        fn into_result(self) -> Result<Self::Ok, Self::Error> {
            self.0
        }

        fn from_error(e: Self::Error) -> Self {
            Combined2(Err(e))
        }

        fn from_ok(t: Self::Ok) -> Self {
            Combined2(Ok(t))
        }
    }

    impl<A, B, C, ERR> Try for Combined3<A, B, C, ERR> {
        type Ok = (A, B, C);
        type Error = Vec<ERR>;

        fn into_result(self) -> Result<Self::Ok, Self::Error> {
            self.0
        }

        fn from_error(e: Self::Error) -> Self {
            Combined3(Err(e))
        }

        fn from_ok(t: Self::Ok) -> Self {
            Combined3(Ok(t))
        }
    }

    impl<A, B, C, D, ERR> Try for Combined4<A, B, C, D, ERR> {
        type Ok = (A, B, C, D);
        type Error = Vec<ERR>;

        fn into_result(self) -> Result<Self::Ok, Self::Error> {
            self.0
        }

        fn from_error(e: Self::Error) -> Self {
            Combined4(Err(e))
        }

        fn from_ok(t: Self::Ok) -> Self {
            Combined4(Ok(t))
        }
    }
}

