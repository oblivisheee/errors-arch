use serde::{Serialize, ser::SerializeStruct as _};

/// Structure that represents an error
#[derive(Debug)]
pub struct ErrorInfo {
    pub r#type: String,
    pub message: String,
    pub details: String,
    pub status: u16,
}

impl ErrorInfo {
    pub fn new(error_type: &str, message: &str, status: u16, details: &str) -> ErrorInfo {
        ErrorInfo {
            r#type: error_type.to_owned(),
            message: message.to_owned(),
            status,
            details: details.to_owned(),
        }
    }
}

impl Serialize for ErrorInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("error", 1)?;
        state.serialize_field("error", &self)?;
        state.end()
    }
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
