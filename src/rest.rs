use std::{collections::HashMap, fmt::Debug};

use crate::mongo;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Serialize, de::DeserializeOwned};

pub struct RestServer {}

impl RestServer {
    pub fn new() -> Self {
        RestServer {}
    }

    pub async fn start<T>(self) -> Result<(), Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
        let mut app = Router::new();
        app = app.route("/", axum::routing::get(RestServer::get_function::<T>));
        app = app.route("/", axum::routing::post(RestServer::post_function::<T>));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

        axum::serve(listener, app).await?;

        Ok(())
    }

    pub async fn post_function<T>(Json(payload): Json<T>) 
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
        match crate::mongo::connect().await {
            Ok(db) => {
                let _ = mongo::insert(&db, payload).await;
            }
            Err(e) => {
                println!("Error connecting to MongoDB: {}", e);
                
            }
        }        
    }

    pub async fn get_function<T>(Query(filter): Query<HashMap<String, String>>) -> impl IntoResponse 
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
       match crate::mongo::connect().await {
            Ok(db) => {
                let r: Result<T, mongodb::error::Error> = mongo::get(&db, filter).await;
                if r.is_ok() {
                    return (StatusCode::OK, format!("Found: {:?}", r.unwrap()));
                } else {
                    return (StatusCode::NOT_FOUND, "No data found".to_string());
                }
            }
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string());
            }
        }        
    }
}
