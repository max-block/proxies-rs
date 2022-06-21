use actix_web::{Scope, web, get};

use crate::template::{Template, TemplateResult};


#[get("/")]
async fn index_page(tpl: web::Data<Template>) -> TemplateResult {
    tpl.render("index.html")
}

pub fn ui_router() -> Scope {
    web::scope("").service(index_page)
}