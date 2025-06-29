use regex::Regex;

pub fn validate_email(email: &str) -> Result<(), String> {
    if !is_valid_email(email) {
        Err(format!("Invalid email: {}", email))
    } else {
        Ok(())
    }
}


/// Email validation according to https://spec.commonmark.org/0.31.2/#email-address
fn is_valid_email(s: &str) -> bool {
    static EMAIL_RE: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";
    Regex::new(EMAIL_RE).unwrap().is_match(s)
}