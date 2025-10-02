use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::mongo;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct RestServer {
    cfg : crate::cfg::Config,
}

#[derive(Clone)]
pub struct AppState {
    db: Arc<mongodb::Database>,
    col: String,
}

#[derive(Serialize)]
pub struct ErrMsg {
    pub msg: String,
}

impl RestServer {
    pub async fn new() -> Self {
        RestServer {
            cfg: crate::cfg::Config::new(),
        }
    }

    pub async fn start<T>(self) -> Result<(), Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
        let db = crate::mongo::connect(self.cfg.connection_string.as_ref(), self.cfg.collection.as_ref())
            .await
            .expect("Failed to connect to MongoDB");

        let mut app = Router::new();
        app = app.route("/", axum::routing::get(RestServer::get_function::<T>));
        app = app.route("/", axum::routing::post(RestServer::post_function::<T>));
        let app: Router<_> = app.with_state(AppState {
            db: Arc::new(db),
            col: self.cfg.collection,
        });

        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.cfg.port)).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }

    pub async fn post_function<T>(State(state): State<AppState>, Json(payload): Json<T>)
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
        let _ = mongo::insert(&state.db, state.col.as_ref(), payload).await;
    }

    pub async fn get_function<T>(
        State(state): State<AppState>,
        Query(filter): Query<HashMap<String, String>>,
    ) -> impl IntoResponse
    where
        T: DeserializeOwned + Serialize + Send + Debug + 'static,
    {
        let r: Result<T, mongodb::error::Error> =
            mongo::get(&state.db, state.col.as_ref(), filter).await;
        match r {
            Ok(value) => (StatusCode::OK, Json(serde_json::to_value(value).unwrap())),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "msg": "No data found" })),
            ),
        }
    }
}
