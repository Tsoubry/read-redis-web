use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use log::error;

use redis::RedisError;

use derive_more::Display;

#[derive(Debug, Display)]
pub enum WebError {
    #[display(fmt = "Redis error: {}", _0.to_string())]
    RedisError(RedisError),
    #[display(fmt = "Invalid json for key \"{}\"", _0)]
    InvalidJsonError(String),
    #[display(fmt = "Gzip unzip error for key \"{}\"", _0)]
    GzipError(String),
}

impl error::ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            WebError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::InvalidJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::GzipError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl WebError {
    pub fn log(self) -> Self {
        error!("{}", &self);
        self
    }
}
