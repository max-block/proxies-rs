use actix_web::web::Json;
use serde::Serialize;
use serde_json::Value;

pub type JsonResult = crate::Result<Json<Value>>;

pub fn json_result<T: Serialize>(value: T) -> JsonResult {
    Ok(Json(serde_json::to_value(value).unwrap())) // TODO: replace unwrap with AppError
}
