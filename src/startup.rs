use std::sync::Arc;

use crate::{app::App, config::Config, scheduler::run_scheduler, server::run_server};

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", " info,actix_web=debug,sqlx=off");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::new();
    let app = App::new(&config).await;
    let app = Arc::new(app);

    run_scheduler(Arc::clone(&app));
    run_server(&config, Arc::clone(&app)).await.unwrap();
    Ok(())
}
