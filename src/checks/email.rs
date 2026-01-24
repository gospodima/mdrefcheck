use regex::Regex;

pub fn validate_email(email: &str) -> Result<(), String> {
    if is_valid_email(email) {
        Ok(())
    } else {
        Err(format!("Invalid email: {email}"))
    }
}

/// Email validation according to <https://spec.commonmark.org/0.31.2/#email-address>
fn is_valid_email(s: &str) -> bool {
    static EMAIL_RE: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";
    Regex::new(EMAIL_RE).unwrap().is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name+tag+sorting@example.co.uk"));
        assert!(validate_email("user@example.com").is_ok());
    }

    #[test]
    fn invalid_emails() {
        assert!(!is_valid_email("not-an-email"));
        assert!(validate_email("not-an-email").is_err());
    }
}
