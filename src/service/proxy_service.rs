use std::sync::Arc;

use crate::db::Db;

pub struct ProxyService {
    db: Arc<Db>,
}

impl ProxyService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}