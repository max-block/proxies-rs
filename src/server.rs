use std::{sync::Arc, time::Duration};

use actix_files::Files;
use actix_web::{get, middleware, web, App as HttpApp, HttpRequest, HttpServer};
use reqwest::Client;
use serde_json::Value;

use crate::{
    app::App,
    config::Config,
    router::{proxy_router, source_router, ui_router},
    template::Template,
    util::JsonResult,
};

#[get("/api-post/{tail_url:.*}")]
async fn api_post(req: HttpRequest) -> JsonResult {
    let mut url = "http://".to_string();
    url.push_str(req.connection_info().host());
    url.push_str(req.uri().to_string().replacen("/api-post/", "/api/", 1).as_str());
    let res: Value = Client::new().post(url).timeout(Duration::from_secs(600)).send().await?.json().await?;

    Ok(web::Json(res))
}

pub async fn run_server(config: &Config, app: Arc<App>) -> std::io::Result<()> {
    let app = web::Data::from(app);

    let tpl = Template::new(config).unwrap();
    let tpl = web::Data::new(tpl);

    log::info!("starting HTTP server at http://localhost:3000");
    HttpServer::new(move || {
        HttpApp::new()
            .wrap(middleware::Logger::default())
            .app_data(tpl.clone())
            .app_data(web::Data::clone(&app))
            .service(Files::new("/static", "static"))
            .service(api_post)
            .service(source_router())
            .service(proxy_router())
            .service(ui_router())
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
