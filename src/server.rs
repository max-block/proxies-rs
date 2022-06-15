use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

use crate::{
    router::{proxy_router, ui_router},
    template::init_tera,
};

pub async fn run() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting http server");

    let tera = init_tera()?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static"))
            .service(proxy_router())
            .service(ui_router())
    })
    .bind("localhost:3000")?
    .run()
    .await?;
    Ok(())
}
