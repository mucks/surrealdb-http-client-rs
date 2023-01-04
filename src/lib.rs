mod client;
mod response;

pub use client::{Client, ClientConfig, Query};
pub use response::{Response, ResponseExt};

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::ResponseExt;
    use crate::{Client, ClientConfig};

    #[derive(Deserialize)]
    struct Test {
        id: String,
        username: String,
    }

    fn config() -> ClientConfig {
        ClientConfig {
            host: "http://localhost:8000".to_string(),
            username: "root".to_string(),
            password: "root".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
        }
    }
    fn client() -> Client {
        Client::new(config()).unwrap()
    }

    #[tokio::test]
    async fn create() {
        let test: Test = client()
            .query("create test set username = $username")
            .bind("username", "test")
            .send()
            .await
            .unwrap()
            .get_result()
            .unwrap();

        assert_eq!(test.username, "test");
        assert!(!test.id.is_empty());
    }

    #[tokio::test]
    async fn select() {
        let _test: Vec<Test> = client()
            .query("SELECT * FROM test")
            .send()
            .await
            .unwrap()
            .get_results()
            .unwrap();
    }

    #[tokio::test]
    async fn invalid_query() {
        if let Err(err) = client()
            .query("creeaate test set username = $username")
            .bind("username", "test")
            .send()
            .await
        {
            assert!(err.to_string().contains("creeaate"));
        } else {
            panic!("Expected error");
        }
    }
}
