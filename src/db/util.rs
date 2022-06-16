use std::borrow::Borrow;

use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, Document};
use mongodb::error::Result as MongoResult;
use mongodb::options::{FindOneOptions, FindOptions};
use mongodb::results::UpdateResult;
use mongodb::{bson, Collection};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct MongoCollection<T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug> {
    collection: Collection<T>,
}

impl<T> MongoCollection<T>
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug,
{
    pub fn new(collection: Collection<T>) -> Self {
        MongoCollection { collection }
    }

    pub async fn insert(&self, doc: &T) -> MongoResult<Bson> {
        Ok(self.collection.insert_one(doc, None).await?.inserted_id)
    }

    pub async fn insert_many(&self, docs: impl IntoIterator<Item = impl Borrow<T>>) -> MongoResult<Vec<Bson>> {
        let r = self.collection.insert_many(docs, None).await?;
        Ok(r.inserted_ids.into_values().collect())
    }

    pub async fn find_one(&self, filter: Document, sort: &str) -> MongoResult<Option<T>> {
        let opts = if sort.starts_with('-') {
            FindOneOptions::builder().sort(doc! {sort.trim_start_matches('-'): -1}).build()
        } else if !sort.is_empty() {
            FindOneOptions::builder().sort(doc! {sort: 1}).build()
        } else {
            FindOneOptions::builder().build()
        };
        self.collection.find_one(filter, opts).await
    }

    pub async fn find(&self, filter: Document, sort: &str, limit: i64) -> MongoResult<Vec<T>> {
        let opts = if sort.starts_with('-') {
            FindOptions::builder().sort(doc! {sort.trim_start_matches('-'): -1}).limit(limit).build()
        } else if !sort.is_empty() {
            FindOptions::builder().sort(doc! {sort: 1}).limit(limit).build()
        } else {
            FindOptions::builder().limit(limit).build()
        };
        let mut cursor = self.collection.find(filter, opts).await?;
        let mut result: Vec<T> = vec![];
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }

    pub async fn find_by_id(&self, id: Bson) -> MongoResult<Option<T>> {
        self.find_one(doc! {"_id": id}, "").await
    }

    pub async fn count(&self, filter: Document) -> MongoResult<u64> {
        self.collection.count_documents(filter, None).await
    }

    pub async fn exists(&self, filter: Document) -> MongoResult<bool> {
        Ok(self.count(filter).await? > 0)
    }

    pub async fn set_by_id(&self, id: Bson, update: Document) -> MongoResult<UpdateResult> {
        self.collection.update_one(doc! {"_id": id}, doc! {"$set": update}, None).await
    }

    pub async fn delete_by_id(&self, id: Bson) -> MongoResult<u64> {
        Ok(self.collection.delete_one(doc! {"_id": id}, None).await?.deleted_count)
    }

    pub async fn delete_many(&self, filter: Document) -> MongoResult<u64> {
        Ok(self.collection.delete_many(filter, None).await?.deleted_count)
    }

    pub async fn delete_one(&self, filter: Document) -> MongoResult<u64> {
        Ok(self.collection.delete_one(filter, None).await?.deleted_count)
    }
}

pub fn bson_object_id_from_str(data: impl AsRef<str>) -> Result<Bson, bson::oid::Error> {
    Ok(Bson::ObjectId(ObjectId::parse_str(data)?))
}
