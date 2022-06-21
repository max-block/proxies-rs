use std::sync::Arc;

use crate::db::Db;

pub struct SourceService {
    db: Arc<Db>,
}

impl SourceService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}