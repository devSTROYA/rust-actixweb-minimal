use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok()
}
