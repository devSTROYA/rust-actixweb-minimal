use actix_web::{HttpResponse, Responder, get, web};

use crate::repositories::user_repository::UserRepo;

#[get("/users")]
pub async fn list_user(repo: web::Data<dyn UserRepo + Send + Sync>) -> impl Responder {
    let _result = repo.find_by_id("");
    let _result = repo.custom_call();
    let users = repo.find_all();

    HttpResponse::Ok().json(users)
}
