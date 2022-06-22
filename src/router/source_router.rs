use actix_web::{delete, get, post, web, Scope};

use crate::{
    app::App,
    db::CreateSource,
    util::{json_result, JsonResult},
};

#[get("")]
async fn get_sources(app: web::Data<App>) -> JsonResult {
    json_result(app.db.find_sources().await?)
}

#[post("")]
async fn create_source(app: web::Data<App>, params: web::Json<CreateSource>) -> JsonResult {
    json_result(app.db.create_source(params.0).await?)
}

#[get("/{id}")]
async fn get_source(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.db.find_source(*id).await?)
}

#[delete("/{id}")]
async fn delete_source() -> JsonResult {
    json_result("delete_source: not impletented yet")
}

#[post("/{id}/check")]
async fn check_source(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.source_service.check(*id).await?)
}

pub fn source_router() -> Scope {
    web::scope("/api/sources")
        .service(get_sources)
        .service(create_source)
        .service(get_source)
        .service(delete_source)
        .service(check_source)
}
