use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{age::Age, email::Email};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: Email,
    pub age: Age,
    pub isActive: bool,
}

impl User {
    pub fn create(email: Email, age: Age) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            email,
            age,
            isActive: true,
        }
    }

    pub fn rehydrate(id: String, email: Email, age: Age, is_active: bool) -> Self {
        User {
            id,
            email,
            age,
            isActive: is_active,
        }
    }
}
