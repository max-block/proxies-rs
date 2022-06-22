use actix_web::{get, web, Scope};
use tera::Context;

use crate::{
    app::App,
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

pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(sources_page).service(proxies_page)
}
