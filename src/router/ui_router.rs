use actix_web::{get, web, Responder, Scope};

use bson::doc;
use tera::{Context, Tera};

use crate::{
    app::App,
    template::{render, HttpResponseResult},
};

#[get("/")]
async fn index_page(tpl: web::Data<Tera>) -> HttpResponseResult {
    render(tpl, "index.html", &Context::new())
}

#[get("/proxies")]
async fn proxies_page(tpl: web::Data<Tera>, app: web::Data<App>) -> HttpResponseResult {
    let mut ctx = Context::new();
    ctx.insert("proxies", &app.db.proxy.find(doc! {}, "host", 0).await?);
    render(tpl, "proxies.html", &ctx)
}

pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(proxies_page)
}
