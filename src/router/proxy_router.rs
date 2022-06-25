use actix_web::{get, web, Scope, post};

use crate::{
    app::App,
    util::{json_result, JsonResult},
};

#[get("")]
async fn get_proxies(app: web::Data<App>) -> JsonResult {
    json_result(app.db.find_proxies().await?)
}

#[get("/test")]
async fn test(app: web::Data<App>) -> JsonResult {
    json_result(app.proxy_service.check_next().await?)
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
    web::scope("/api/proxies").service(get_proxies).service(test).service(get_proxy).service(check_proxy)
}
