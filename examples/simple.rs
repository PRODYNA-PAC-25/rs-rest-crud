use serde::Serialize;

#[tokio::main]
async fn main() {
   
    #[derive(serde::Deserialize, Serialize, Debug)]
    pub struct Participant {
        pub name: String,
        pub age: u8,
    }

    let _ = rs_rest_mongo::rest::RestServer::new().start::<Participant>().await;
}
