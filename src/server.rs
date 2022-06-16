use actix_files::Files;
use actix_web::{middleware, web, App as HttpApp, HttpServer};

use crate::{
    app::App,
    config::AppConfig,
    router::{proxy_router, source_router, ui_router},
    template::init_tera,
};

pub async fn run() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting http server");
    let config = AppConfig::get();

    let tera = init_tera()?;
    let app = App::new(&config.database_url).await.unwrap();
    let app = web::Data::new(app);
    HttpServer::new(move || {
        HttpApp::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(app.clone())
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static"))
            .service(source_router())
            .service(proxy_router())
            .service(ui_router())
    })
    .bind("localhost:3000")?
    .run()
    .await?;
    Ok(())
}
