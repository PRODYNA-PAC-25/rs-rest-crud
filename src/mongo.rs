use std::{collections::HashMap, fmt::Debug};

use mongodb::bson::to_bson;
use serde::{Serialize, de::DeserializeOwned};

const COLLECTION: &str = "pac";

pub async fn connect() -> Result<mongodb::Database, mongodb::error::Error> {
    let con_str = match std::env::var("CONNECTION_STRING") {
        Ok(v) => v,
        Err(_) => "mongodb://127.0.0.1:27017/?directConnection=true&serverSelectionTimeoutMS=2000"
            .to_string(),
    };

    let client_options = mongodb::options::ClientOptions::parse(&con_str).await?;
    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database(COLLECTION);
    Ok(db)
}

pub async fn insert<T>(db: &mongodb::Database, payload: T) -> Result<(), mongodb::error::Error>
where
    T: DeserializeOwned + Serialize + Send + Debug + 'static,
{
    let collection: mongodb::Collection<_> = db.collection(COLLECTION);

    let doc = to_bson(&payload)?;
    collection.insert_one(doc).await?;

    Ok(())
}

pub async fn get<T>(
    db: &mongodb::Database,
    filter: HashMap<String, String>,
) -> Result<T, mongodb::error::Error>
where
    T: DeserializeOwned + Serialize + Send + Debug + 'static,
{
    let collection = db.collection::<mongodb::bson::Document>(COLLECTION);

    let r: Option<_> = collection.find_one(convert_hashmap_to_bson(filter)).await?;

    match r {
        Some(doc) => {
            let result: T = mongodb::bson::from_document(doc)?;
            Ok(result)
        }
       None => {
            Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No data found",
            )))
        }
    }
}

fn convert_hashmap_to_bson(map: HashMap<String, String>) -> mongodb::bson::Document {
    let mut doc = mongodb::bson::Document::new();
    for (key, value) in map {
        match key {
            key if key != "" => {
                doc.insert(key, value);
            }
            _ => { /* skip empty keys */ }
        }
    }
    doc
}
