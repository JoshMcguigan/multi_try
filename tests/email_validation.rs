use multi_try::MultiTry;

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
    EmailAddress,
    RecipientEmailAddress,
    SenderEmailAddress,
    Subject,
    Body,
}

fn validate_address(address: &str) -> Result<&str, EmailValidationErr> {
    if address.contains('@') {
        Ok(address)
    } else {
        Err(EmailValidationErr::EmailAddress)
    }
}

fn validate_subject(subject: &str) -> Result<&str, EmailValidationErr> {
    if subject.len() > 5 {
        Ok(subject)
    } else {
        Err(EmailValidationErr::Subject)
    }
}

fn validate_body(body: &str) -> Result<&str, EmailValidationErr> {
    if body.len() > 10 {
        Ok(body)
    } else {
        Err(EmailValidationErr::Body)
    }
}

fn validate_email(email: Email) -> Result<ValidatedEmail, Vec<EmailValidationErr>> {
    let (to, from, subject, body) = validate_address(email.to)
        .map_err(|_| EmailValidationErr::RecipientEmailAddress)
        .and_try(validate_address(email.from).map_err(|_| EmailValidationErr::SenderEmailAddress))
        .and_try(validate_subject(email.subject))
        .and_try(validate_body(email.body))?;

    Ok(ValidatedEmail {
        to,
        from,
        subject,
        body,
    })
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
            EmailValidationErr::RecipientEmailAddress,
            EmailValidationErr::SenderEmailAddress,
            EmailValidationErr::Subject,
            EmailValidationErr::Body,
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
        let expected = Err(vec![EmailValidationErr::Body]);

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
