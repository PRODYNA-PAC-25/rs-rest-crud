#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub connection_string: String,
    pub collection: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: port(),
            connection_string: connection_string(),
            collection: collection(),
        }
    }
}

fn connection_string() -> String {
    std::env::var("CONNECTION_STRING").unwrap_or_else(|_| {
        "mongodb://127.0.0.1:27017/?directConnection=true&serverSelectionTimeoutMS=2000".to_string()
    })
}

fn port() -> u16 {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    port.parse::<u16>().unwrap_or(3000)
}

fn collection() -> String {
    std::env::var("COLLECTION").unwrap_or_else(|_| "participant".to_string())
}
