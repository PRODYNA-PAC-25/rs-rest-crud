use std::borrow::Cow;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub connection_string: Cow<'static, str>,
    pub collection: Cow<'static, str>,
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

fn connection_string() -> Cow<'static, str> {
    let con_str = std::env::var("CONNECTION_STRING").unwrap_or_else(|_| {
        "mongodb://127.0.0.1:27017/?directConnection=true&serverSelectionTimeoutMS=2000".to_string()
    });
    con_str.into()
}

fn port() -> u16 {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    port.parse::<u16>().unwrap_or(3000)
}

fn collection() -> Cow<'static, str> {
    let collection = std::env::var("COLLECTION").unwrap_or_else(|_| "pac".to_string());
    collection.into()
}
