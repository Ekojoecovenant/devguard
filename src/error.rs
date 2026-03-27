pub struct DevGuardError {
    pub key: String,
    pub rule: String,
    pub message: String,
}

impl DevGuardError {
    pub fn new(key: String, rule: String, message: String) -> Self {
        DevGuardError { key, rule, message }
    }
}
