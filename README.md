# simple surrealdb http client written in rust


```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Test {
    id: String,
    username: String,
}

let cfg = ClientConfig {
    host: "http://localhost:8000".to_string(),
    username: "root".to_string(),
    password: "root".to_string(),
    namespace: "test".to_string(),
    database: "test".to_string(),
};

let client = Client::new(cfg).unwrap();

let test: Test = client
    .query("create test set username = $username")
    .bind("username", "test")
    .send()
    .await
    .unwrap()
    .get_result()
    .unwrap();
```