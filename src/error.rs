use actix_web::ResponseError;
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

    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("parse objectId error")]
    ParseObjectId(#[from] bson::oid::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl ResponseError for AppError {}