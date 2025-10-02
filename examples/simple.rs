use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    #[derive(serde::Deserialize, Serialize, Debug)]
    pub struct Participant {
        pub name: String,
        pub age: u8,
    }

    let _ = rs_rest_crud::rest::RestServer::new().await.start::<Participant>().await?;
    Ok(())
}
