use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

pub type JsonResult = crate::error::Result<Json<Value>>;

pub fn json_result<T: Serialize>(value: T) -> JsonResult {
    Ok(Json(serde_json::to_value(value).unwrap())) // TODO: replace unwrap with AppError
}


#[macro_export]
macro_rules! synchronized {
    () => {
        lazy_static::lazy_static! {
            static ref LOCK: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
        }
        let _guard = LOCK.lock().unwrap();
    };
}

pub fn utc_delta_seconds(value: i16) -> DateTime<Utc> {
    Utc::now().checked_add_signed(chrono::Duration::seconds(value as i64)).unwrap()
}