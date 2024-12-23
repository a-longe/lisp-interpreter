pub struct Error {
    reason: String
    // position?
}

pub fn create_error(msg: &str) -> Error {
    Error {
        reason: msg.to_string()
    }
}
