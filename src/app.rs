use crate::db::Db;
use crate::service::{ProxyService, SourceService};
use std::sync::Arc;

pub struct App {
    pub db: Arc<Db>,
    pub source_service: SourceService,
    pub proxy_service: ProxyService,
}

impl App {
    pub async fn new(database_url: &str) -> crate::Result<Self> {
        let db = Arc::new(Db::new(database_url).await?);
        let source_service = SourceService::new(db.clone());
        let proxy_service = ProxyService::new(db.clone());
        Ok(Self { db, source_service, proxy_service })
    }
}
