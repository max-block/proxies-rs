use actix_web::{get, web, Responder, Scope};

#[get("")]
async fn index_page() -> impl Responder {
    "proxies index"
}

#[get("/t1")]
async fn t1_page() -> impl Responder {
    "proxies t1"
}

pub fn proxy_router() -> Scope {
    web::scope("/api/proxies").service(index_page).service(t1_page)
}
