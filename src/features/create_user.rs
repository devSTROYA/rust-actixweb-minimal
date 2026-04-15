use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{age::Age, email::Email, user::User};
use crate::repositories::user_repository::UserRepo;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    email: String,
    age: i32,
}

#[post("/users")]
pub async fn create_user(
    request: web::Json<CreateUserRequest>,
    repo: web::Data<dyn UserRepo + Send + Sync>,
) -> impl Responder {
    let mut errors = Vec::new();
    let email = match Email::create(request.email.to_owned()) {
        Ok(email) => Some(email),
        Err(error) => {
            errors.push(error);
            None
        }
    };

    let age = match Age::create(request.age) {
        Ok(age) => Some(age),
        Err(error) => {
            errors.push(error);
            None
        }
    };

    if !errors.is_empty() {
        return HttpResponse::BadRequest().json(errors);
    }

    // unused, only to make created function not get warned
    let _email = Email::rehydrate(request.email.to_owned());
    let _age = Age::rehydrate(request.age);
    let _user = User::rehydrate(Uuid::new_v4().to_string(), _email, _age, true);

    let user = User::create(email.unwrap(), age.unwrap());
    repo.save(user.clone()).unwrap();

    HttpResponse::Ok().json(user)
}
