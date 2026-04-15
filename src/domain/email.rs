use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn create(value: String) -> Result<Self, String> {
        if value.contains('@') {
            Ok(Email(value))
        } else {
            return Err(String::from("Invalid email format"));
        }
    }

    pub fn rehydrate(value: String) -> Self {
        Email(value)
    }
}
