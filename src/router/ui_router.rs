use actix_web::{
    get,
    web::{self, Data},
    Responder, Scope,
};
use chrono::Utc;
use tera::{Context, Tera};

use crate::template::render;

#[get("/")]
async fn index_page(tpl: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("t1", &Utc::now());
    render(tpl, "index.html", &ctx)
}

#[get("/t1")]
async fn t1_page() -> impl Responder {
    "t1"
}

pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(t1_page)
}
