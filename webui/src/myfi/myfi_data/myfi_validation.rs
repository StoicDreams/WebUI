use regex::Regex;

pub fn validate_email(email: &str) -> Option<String> {
    let email = email.trim();
    if email.is_empty() {
        return Some(String::from("Email address is required"));
    }
    if let Ok(re) = Regex::new(r"^[a-zA-Z0-9.+-_]+@[a-zA-Z0-9-_]+\.[a-zA-Z0-9.-_]+$") {
        if !re.is_match(email) {
            return Some(String::from("Invalid email address"));
        }
    }
    None
}

pub fn validate_password(password: &str) -> Option<String> {
    let password = password.trim();
    if password.is_empty() {
        return Some(String::from("Password is required"));
    }
    if password.len() < 18 {
        return Some(String::from("Password must be at least 18 characters"));
    }
    None
}
