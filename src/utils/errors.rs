use serde_json::Error as SerdeError;
use std::fmt;

#[derive(Debug)]
pub enum FetchError {
    ParseError(String),     // For issues like timestamp parsing
    NetworkError(String),   // For network-related issues
    DatabaseError(String),  // For database-related issues
    SerdeError(SerdeError), // For serde deserialization errors
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FetchError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            FetchError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            FetchError::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl std::error::Error for FetchError {}
