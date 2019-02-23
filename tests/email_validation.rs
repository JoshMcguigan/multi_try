#[derive(Debug, PartialEq)]
struct Email<'a> {
    to: &'a str,
    from: &'a str,
    subject: &'a str,
    body: &'a str,
}

#[derive(Debug, PartialEq)]
struct ValidatedEmail<'a> {
    to: &'a str,
    from: &'a str,
    subject: &'a str,
    body: &'a str,
}

#[derive(Debug, PartialEq)]
enum EmailValidationErr {
    InvalidEmailAddress,
    InvalidRecipientEmailAddress,
    InvalidSenderEmailAddress,
    InvalidSubject,
    InvalidBody,
}

fn validate_address(address: &str) -> Result<&str, EmailValidationErr> {
    if address.contains("@") {
        Ok(address)
    } else {
        Err(EmailValidationErr::InvalidEmailAddress)
    }
}

fn validate_subject(subject: &str) -> Result<&str, EmailValidationErr> {
    if subject.len() > 5 {
        Ok(subject)
    } else {
        Err(EmailValidationErr::InvalidSubject)
    }
}

fn validate_body(body: &str) -> Result<&str, EmailValidationErr> {
    if body.len() > 10 {
        Ok(body)
    } else {
        Err(EmailValidationErr::InvalidBody)
    }
}

fn validate_email(email: Email) -> Result<ValidatedEmail, Vec<EmailValidationErr>> {
    let (to, from, subject, body) =
        multi_try::and(
            validate_address(email.to)
                .map_err(|_| EmailValidationErr::InvalidRecipientEmailAddress),
            validate_address(email.from)
                .map_err(|_| EmailValidationErr::InvalidSenderEmailAddress)
        )
        .and(
            validate_subject(email.subject)
        )
        .and(
            validate_body(email.body)
        )
        .into_result()?;

    Ok(ValidatedEmail { to, from, subject, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_err() {
        let a = Email {
            to: "Tom",
            from: "Mary",
            subject: "s",
            body: "b",
        };

        let result = validate_email(a);
        let expected = Err(vec![
            EmailValidationErr::InvalidRecipientEmailAddress,
            EmailValidationErr::InvalidSenderEmailAddress,
            EmailValidationErr::InvalidSubject,
            EmailValidationErr::InvalidBody,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn body_err() {
        let a = Email {
            to: "Tom@mail.com",
            from: "Mary@mail.com",
            subject: "good morning",
            body: "b",
        };

        let result = validate_email(a);
        let expected = Err(vec![
            EmailValidationErr::InvalidBody,
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn all_ok() {
        let a = Email {
            to: "Tom@mail.com",
            from: "Mary@mail.com",
            subject: "good morning",
            body: "Isn't it a lovely morning?!",
        };

        let result = validate_email(a);
        let expected = Ok(ValidatedEmail {
            to: "Tom@mail.com",
            from: "Mary@mail.com",
            subject: "good morning",
            body: "Isn't it a lovely morning?!",
        });

        assert_eq!(expected, result);
    }
}