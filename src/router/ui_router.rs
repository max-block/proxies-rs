use actix_web::{get, post, web, Responder, Scope};
use tera::Context;

use crate::{
    app::App,
    db::CreateSource,
    template::{Template, TemplateResult},
};

#[get("/")]
async fn index_page(tpl: web::Data<Template>) -> TemplateResult {
    tpl.render("index.html")
}

#[get("/sources")]
async fn sources_page(tpl: web::Data<Template>, app: web::Data<App>) -> TemplateResult {
    let sources = app.db.find_sources().await?;
    let mut ctx = Context::new();
    ctx.insert("sources", &sources);
    tpl.render_with_ctx("sources.html", ctx)
}

#[get("/proxies")]
async fn proxies_page(tpl: web::Data<Template>, app: web::Data<App>) -> TemplateResult {
    let proxies = app.db.find_proxies().await?;
    let mut ctx = Context::new();
    ctx.insert("proxies", &proxies);
    tpl.render_with_ctx("proxies.html", ctx)
}

#[post("/create_source")]
async fn create_source(app: web::Data<App>, params: web::Form<CreateSource>) -> impl Responder {
    app.db.create_source(params.0).await.unwrap();
    "done"
}

pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(sources_page).service(proxies_page).service(create_source)
}
