use actix_web::{Responder, ResponseError, HttpResponse, http::header::ContentType};
use mongodb::bson;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("empty database")]
    EmptyDatabase,

    #[error("object not found")]
    NotFound,

    #[error("mongo error")]
    MongoError(#[from] mongodb::error::Error),

    #[error("actix error")]
    ActixError(#[from] actix_web::Error),

    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("parse objectId error")]
    ParseObjectId(#[from] bson::oid::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
