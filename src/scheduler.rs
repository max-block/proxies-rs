use std::{sync::Arc, time::Duration};

use clokwerk::{AsyncScheduler, TimeUnits};

use crate::app::App;

pub fn run_scheduler(app: Arc<App>) {
    let mut scheduler = AsyncScheduler::new();

    let app_job = Arc::clone(&app);
    scheduler.every(60.seconds()).run(move || {
        let app = app_job.clone();
        async move {
            app.source_service.check_next().await.unwrap();
        }
    });

    let app_job = Arc::clone(&app);
    scheduler.every(2.seconds()).run(move || {
        let app = app_job.clone();
        async move {
            app.proxy_service.check_next().await.unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });
}
