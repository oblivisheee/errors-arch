use serde::Serialize;
/// Structure that represents an error
#[derive(Debug)]
pub struct ErrorInfo {
    pub r#type: String,
    pub message: String,
    pub details: String,
    pub timestamp: String,
    pub status: u16,
}

impl ErrorInfo {
    pub fn new(error_type: &str, message: &str, status: u16, details: &str) -> ErrorInfo {
        ErrorInfo {
            r#type: error_type.to_owned(),
            message: message.to_owned(),
            status,
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: details.to_owned(),
        }
    }
}

impl Serialize for ErrorInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct Inner<'a> {
            r#type: &'a str,
            message: &'a str,
            details: &'a str,
            timestamp: &'a str,
            status: u16,
        }

        #[derive(serde::Serialize)]
        struct Wrapper<'a> {
            error: Inner<'a>,
        }

        let inner = Inner {
            r#type: &self.r#type,
            message: &self.message,
            details: &self.details,
            timestamp: &self.timestamp,
            status: self.status,
        };
        let wrapper = Wrapper { error: inner };
        wrapper.serialize(serializer)
    }
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_error_info_new() {
        let error = ErrorInfo::new("TypeError", "An error occurred", 400, "Invalid input");
        assert_eq!(error.r#type, "TypeError");
        assert_eq!(error.message, "An error occurred");
        assert_eq!(error.status, 400);
        assert_eq!(error.details, "Invalid input");
        println!("{}", error);
    }

    #[test]
    fn test_error_info_serialize() {
        let error = ErrorInfo::new("TypeError", "An error occurred", 400, "Invalid input");
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "type": "TypeError",
            "message": "An error occurred",
            "details": "Invalid input",
            "status": 400
        });
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            expected
        );
    }

    #[test]
    fn test_error_info_display() {
        let error = ErrorInfo::new("TypeError", "An error occurred", 400, "Invalid input");
        let display = format!("{}", error);
        let expected = serde_json::to_string_pretty(&json!({
            "type": "TypeError",
            "message": "An error occurred",
            "details": "Invalid input",
            "status": 400
        }))
        .unwrap();
        assert_eq!(display, expected);
    }
}
