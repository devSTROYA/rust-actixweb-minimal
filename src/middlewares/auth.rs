use core::fmt;

use actix_web::{
    Error, HttpMessage, HttpResponse, ResponseError,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Next,
};
use serde::Serialize;

// 1. Create a struct to hold the data you want to pass to the handler
#[derive(Debug, Clone)]
struct AuthenticatedUser {
    id: String,
    role: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// 2. An Enum representing the different things that can go wrong
#[derive(Debug)]
pub enum AuthError {
    MissingHeader,
    InvalidFormat,
    InvalidToken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Authentication failed")
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AuthError::MissingHeader => "No token provided.",
            AuthError::InvalidFormat => "Token malformed.",
            AuthError::InvalidToken => "Invalid or expired token.",
        };

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: "Unauthorized".to_string(),
            message: message.to_string(),
        })
    }
}

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_header = req.headers().get("Authorization");

    let auth_header = match auth_header {
        Some(header) => header.to_str().unwrap_or(""),
        None => return Err(AuthError::MissingHeader.into()),
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidFormat.into());
    }

    let token = auth_header.replace("Bearer ", "");

    if token != "secret_token" {
        return Err(AuthError::InvalidToken.into());
    }

    let user = AuthenticatedUser {
        id: "user_123".to_string(),
        role: "admin".to_string(),
    };

    req.extensions_mut().insert(user);

    let res = next.call(req).await?;

    Ok(res)
}
