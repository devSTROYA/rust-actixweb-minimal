use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize)]
pub struct Age(i32);

impl Age {
    pub fn create(value: i32) -> Result<Self, String> {
        if value < 18 {
            return Err(String::from("Age must be at least 18"));
        }

        Ok(Age(value))
    }

    pub fn rehydrate(value: i32) -> Self {
        Age(value)
    }
}
