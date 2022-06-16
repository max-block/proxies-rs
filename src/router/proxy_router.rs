use actix_web::{
    get, post,
    web::{self, Data, Path},
    Scope,
};
use bson::{doc, oid::ObjectId};

use crate::{
    app::App,
    db::bson_object_id_from_str,
    util::{json_result, JsonResult},
};

#[get("")]
async fn get_proxies(app: Data<App>) -> JsonResult {
    json_result(app.db.proxy.find(doc! {}, "_id", 0).await?)
}

#[get("/{id}")]
async fn get_proxy(app: Data<App>, id: Path<String>) -> JsonResult {
    json_result(app.db.proxy.find_by_id(bson_object_id_from_str(&*id)?).await?)
}

#[post("/{id}/check")]
async fn check_proxy(app: Data<App>, id: Path<String>) -> JsonResult {
    json_result(app.proxy_service.check(ObjectId::parse_str(&*id)?).await?)
}

pub fn proxy_router() -> Scope {
    web::scope("/api/proxies").service(get_proxies).service(get_proxy).service(check_proxy)
}
