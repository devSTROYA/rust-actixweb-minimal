use std::fmt::Error;

pub trait Repo<T> {
    fn find_by_id(&self, id: &str) -> Option<T>;
    fn find_all(&self) -> Vec<T>;
    fn save(&self, item: T) -> Result<(), Error>;
}
