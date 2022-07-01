use std::{sync::Arc, time::Duration};

use clokwerk::{AsyncScheduler, TimeUnits};

use crate::app::App;

pub fn run_scheduler(app: Arc<App>) {
    let mut scheduler = AsyncScheduler::new();

    let app_job1 = Arc::clone(&app);
    let app_job2 = Arc::clone(&app);

    scheduler.every(60.seconds()).run(move || {
        let app = app_job1.clone();
        async move {
            app.source_service.check_next().await.unwrap();
        }
    });
    scheduler.every(2.seconds()).run(move || {
        let app = app_job2.clone();
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
