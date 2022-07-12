use actix_web::{get, post, web, Scope};
use serde_json::json;

use crate::{
    app::App,
    util::{json_result, JsonResult},
};

#[get("")]
async fn get_proxies(app: web::Data<App>) -> JsonResult {
    json_result(app.db.find_proxies().await?)
}

#[get("/live")]
async fn get_live_proxies(app: web::Data<App>) -> JsonResult {
    let proxies = app.db.find_live_proxies().await?;
    let proxies: Vec<String> = proxies.into_iter().map(|p| p.url()).collect();
    json_result(json!({ "proxies": proxies }))
}

#[get("/{id}")]
async fn get_proxy(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.db.find_proxy(*id).await?)
}

#[post("/{id}/check")]
async fn check_proxy(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.proxy_service.check(*id).await?)
}

pub fn proxy_router() -> Scope {
    web::scope("/api/proxies").service(get_proxies).service(get_live_proxies).service(get_proxy).service(check_proxy)
}
