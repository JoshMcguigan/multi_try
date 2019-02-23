#[derive(Debug, PartialEq)]
struct A {
    b: Option<i32>,
    c: Option<i32>,
    d: Option<i32>,
    e: Option<u32>,
}

#[derive(Debug, PartialEq)]
struct ValidatedA {
    b: i32,
    c: i32,
    d: i32,
    e: u32,
}

#[derive(Debug, PartialEq)]
enum ValidationError {
    MissingB,
    MissingC,
    MissingD,
    MissingE,
}

fn validate(a: A) -> Result<ValidatedA, Vec<ValidationError>> {
    let (b, c, d, e) =
        multi_try::and(
            a.b.ok_or(ValidationError::MissingB),
            a.c.ok_or(ValidationError::MissingC)
        )
        .and(a.d.ok_or(ValidationError::MissingD))
        .and(a.e.ok_or(ValidationError::MissingE))?;

    Ok(ValidatedA { b, c, d, e })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_empty() {
        let a = A {
            b: None,
            c: None,
            d: None,
            e: None,
        };

        let result = validate(a);
        let expected = Err(vec![
            ValidationError::MissingB,
            ValidationError::MissingC,
            ValidationError::MissingD,
            ValidationError::MissingE,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn b_empty() {
        let a = A {
            b: None,
            c: Some(1),
            d: Some(1),
            e: Some(2),

        };

        let result = validate(a);
        let expected = Err(vec![ValidationError::MissingB]);

        assert_eq!(expected, result);
    }

    #[test]
    fn none_empty() {
        let a = A {
            b: Some(1),
            c: Some(2),
            d: Some(3),
            e: Some(4),
        };

        let result = validate(a);
        let expected = Ok(ValidatedA { b: 1, c: 2, d: 3, e: 4 });

        assert_eq!(expected, result);
    }
}