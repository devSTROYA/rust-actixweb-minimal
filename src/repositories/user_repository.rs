use std::sync::Mutex;

use crate::{core::repo::Repo, domain::user::User};

static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());

pub trait UserRepo: Repo<User> {
    fn custom_call(&self) -> Option<User>;
}

pub struct InMemoryUserRepo;
impl InMemoryUserRepo {
    pub fn new() -> Self {
        InMemoryUserRepo
    }
}
impl Repo<User> for InMemoryUserRepo {
    fn find_by_id(&self, id: &str) -> Option<User> {
        let users = USERS.lock().unwrap();
        let selected_user = users.iter().find(|user| user.id == id);
        if selected_user.is_none() {
            return None;
        }

        Some(selected_user.unwrap().clone())
    }

    fn save(&self, item: User) -> Result<(), std::fmt::Error> {
        let mut users = USERS.lock().unwrap();
        if let Some(existing_user) = users.iter_mut().find(|user| user.id == item.id) {
            existing_user.email = item.email;
            existing_user.age = item.age;
            existing_user.isActive = item.isActive;
        } else {
            users.push(item);
        }

        Ok(())
    }

    fn find_all(&self) -> Vec<User> {
        let users = USERS.lock().unwrap().clone();
        users.into_iter().collect()
    }
}

impl UserRepo for InMemoryUserRepo {
    fn custom_call(&self) -> Option<User> {
        let users = USERS.lock().unwrap();
        let selected_user = users.iter().find(|user| user.id == '1'.to_string());
        if selected_user.is_none() {
            return None;
        }

        Some(selected_user.unwrap().clone())
    }
}
