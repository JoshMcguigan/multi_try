use multi_try::MultiTry;

#[derive(Debug, PartialEq)]
struct A {
    b: Result<i32, MyErr>,
    c: Result<i64, MyErr>,
    d: Result<f32, MyErr>,
}

#[derive(Debug, PartialEq)]
struct ValidatedA {
    b: i32,
    c: i64,
    d: f32,
}

#[derive(Debug, PartialEq)]
enum MyErr {
    FailedB,
    FailedC,
    FailedD,
}

fn validate(a: A) -> Result<ValidatedA, Vec<MyErr>> {
    let (b, c, d) = a.b.and_try(a.c).and_try(a.d)?;

    Ok(ValidatedA { b, c, d })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_err() {
        let a = A {
            b: Err(MyErr::FailedB),
            c: Err(MyErr::FailedC),
            d: Err(MyErr::FailedD),
        };

        let result = validate(a);
        let expected = Err(vec![
            MyErr::FailedB,
            MyErr::FailedC,
            MyErr::FailedD,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn c_err() {
        let a = A {
            b: Ok(1),
            c: Err(MyErr::FailedC),
            d: Ok(3.0),
        };

        let result = validate(a);
        let expected = Err(vec![
            MyErr::FailedC,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn all_ok() {
        let a = A {
            b: Ok(1),
            c: Ok(2),
            d: Ok(3.0),
        };

        let result = validate(a);
        let expected = Ok(ValidatedA { b: 1, c: 2, d: 3.0 });

        assert_eq!(expected, result);
    }
}
