use mongodb::{bson::{self, to_bson, Bson, Document}, options::ClientOptions, Collection, Database};
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::HashMap, fmt::Debug};

pub async fn connect(
    connection_str: &str,
    collection: &str,
) -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(connection_str).await?;
    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database(collection);
    Ok(db)
}

pub async fn insert<T>(
    db: &Database,
    collection: &str,
    payload: T,
) -> Result<(), mongodb::error::Error>
where
    T: DeserializeOwned + Serialize + Send + Debug + 'static,
{
    let collection: Collection<_> = db.collection(collection);

    let doc = to_bson(&payload)?;
    collection.insert_one(doc).await?;

    Ok(())
}

pub async fn get<T>(
    db: &mongodb::Database,
    collection: &str,
    filter: HashMap<String, String>,
) -> Result<T, mongodb::error::Error>
where
    T: DeserializeOwned + Serialize + Send + Debug + 'static,
{
    let collection = db.collection::<Document>(collection);

    let r: Option<_> = collection.find_one(convert_hashmap_to_bson(filter)).await?;

    match r {
        Some(doc) => {
            let result: T = bson::from_document(doc)?;
            Ok(result)
        }
        None => Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No data found",
        ))),
    }
}

fn convert_hashmap_to_bson(filter: HashMap<String, String>) -> Document {
    filter.into_iter().map(|(k, v)| (k, Bson::String(v))).collect()
}