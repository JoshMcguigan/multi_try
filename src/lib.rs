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

impl<T, U, RT, ERR> MultiTry<RT, ERR> for Result<(T, U), Vec<ERR>> {
    type Output = Result<(T, U, RT), Vec<ERR>>;

    fn and_try(self, other: Result<RT, ERR>) -> Self::Output {
        match (self, other) {
            (Ok((a, b)), Ok(c)) => Ok((a, b, c)),
            (Ok(_), Err(ec)) => Err(vec![ec]),
            (Err(errs), Ok(_)) => Err(errs),
            (Err(mut errs), Err(ec)) => Err({
                errs.push(ec);
                errs
            }),
        }
    }
}

impl<T, U, V, RT, ERR> MultiTry<RT, ERR> for Result<(T, U, V), Vec<ERR>> {
    type Output = Result<(T, U, V, RT), Vec<ERR>>;

    fn and_try(self, other: Result<RT, ERR>) -> Self::Output {
        match (self, other) {
            (Ok((a, b, c)), Ok(d)) => Ok((a, b, c, d)),
            (Ok(_), Err(ec)) => Err(vec![ec]),
            (Err(errs), Ok(_)) => Err(errs),
            (Err(mut errs), Err(ec)) => Err({
                errs.push(ec);
                errs
            }),
        }
    }
}

impl<T, U, V, W, RT, ERR> MultiTry<RT, ERR> for Result<(T, U, V, W), Vec<ERR>> {
    type Output = Result<(T, U, V, W, RT), Vec<ERR>>;

    fn and_try(self, other: Result<RT, ERR>) -> Self::Output {
        match (self, other) {
            (Ok((a, b, c, d)), Ok(e)) => Ok((a, b, c, d, e)),
            (Ok(_), Err(ec)) => Err(vec![ec]),
            (Err(errs), Ok(_)) => Err(errs),
            (Err(mut errs), Err(ec)) => Err({
                errs.push(ec);
                errs
            }),
        }
    }
}
