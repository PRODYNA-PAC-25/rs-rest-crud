# rs-rest-crud

`rs-rest-crud` was created as live coding session for demonstrating rust with axum, tokio and a mongo database.

# Configuration

- PORT: default 3000
- CONNECTION_STRING: default mongodb://127.0.0.1:27017/?directConnection=true&serverSelectionTimeoutMS=2000
- COLLECTION: default participant


# Run

cargo run --examples simple

# Example

```rust
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
```



# Sample Requests

## Create

```bash
curl --request POST \
  --url http://localhost:3000/ \
  --header 'Content-Type: application/json' \
  --data '{
	"name" : "participant",
	"age" : 53
}'
```

## bash

```curl
curl --request GET \
  --url 'http://localhost:3000/?name=participant'
```