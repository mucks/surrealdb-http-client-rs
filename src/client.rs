use anyhow::{anyhow, Result};
use hyper::{client::HttpConnector, Body, HeaderMap, Method, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use crate::response::Response;

#[derive(Debug, Default, Clone)]
pub struct ClientConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug)]
pub struct Query<'a> {
    sql: String,
    client: &'a Client,
}

impl<'a> Query<'a> {
    pub fn new(sql: String, client: &'a Client) -> Self {
        Self { sql, client }
    }

    pub fn bind(self, key: &str, value: &str) -> Self {
        let v = &format!("\"{}\"", value);
        let k = &format!("${}", key);

        Self {
            sql: self.sql.replace(k, v),
            client: self.client,
        }
    }
    pub async fn send(self) -> Result<Vec<Response>> {
        self.client.post(self.sql).await
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    _config: ClientConfig,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    headers: HeaderMap,
    url: String,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http2()
            .build();

        let client = hyper::Client::builder().build(https);
        let auth = base64::encode(format!("{}:{}", config.username, config.password));
        let url = format!("{}/sql", config.host);

        let mut headers = HeaderMap::new();
        headers.append("Authorization", format!("Basic {auth}").parse()?);
        headers.append("Accept", "application/json".parse()?);
        headers.append("Content-Type", "text/plain".parse()?);
        headers.append("NS", config.namespace.parse()?);
        headers.append("DB", config.database.parse()?);

        Ok(Self {
            url,
            _config: config,
            client,
            headers,
        })
    }

    pub fn query(&self, sql: &str) -> Query {
        Query::new(sql.to_string(), self)
    }

    async fn post(&self, sql: String) -> Result<Vec<Response>> {
        let mut req = Request::builder()
            .method(Method::POST)
            .uri(&self.url)
            .body(Body::from(sql.clone()))?;
        *req.headers_mut() = self.headers.clone();

        let resp = self.client.request(req).await?;
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let out = String::from_utf8(body_bytes.to_vec())?;
        let resp = match serde_json::from_str::<Vec<Response>>(&out) {
            Ok(resp) => {
                if resp.is_empty() {
                    return Err(anyhow!("No results found for query: {}", sql));
                }
                if resp[0].status != "OK" {
                    let err = serde_json::to_string(&resp[0])?;
                    return Err(anyhow!("response: {}\nsql: '{}'", err, sql));
                }
                resp
            }
            Err(_) => return Err(anyhow!("{}", out)),
        };
        Ok(resp)
    }
}
