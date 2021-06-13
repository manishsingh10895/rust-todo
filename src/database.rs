use crate::todo;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson, MongoQueryError};
use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client, Collection};
use std::io::Result;


#[derive(Debug, Clone)]
pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;

        client_options.app_name = Some(String::from("todo"));

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_todos(&self) -> Result<Vec<todo::TodoItem>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(Error)?;

        let mut result: Vec<todo::TodoItem> = Vec::new();

        while let Some(doc) = cursor.next().await {
            result.push(self)
        }
    }

    fn doc_to_item(&self, doc: &Document) -> Result<todo::TodoItem> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let completed = doc.get_bool("completed")?;

        let item = todo::TodoItem {
            _id: String::from(id),
            name: String::from(name),
            completed: completed,
        };

        Ok(item)
    }
}
