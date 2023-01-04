use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize};

pub trait ResponseExt {
    fn get_result<T>(&self) -> Result<T>
    where
        T: DeserializeOwned;
    fn get_results<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned;
}

impl ResponseExt for Vec<Response> {
    fn get_result<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if let Some(result) = &self
            .get(0)
            .ok_or_else(|| anyhow!("response is empty"))?
            .result
        {
            if let Some(first) = result.first() {
                return Ok(serde_json::from_value(first.clone())?);
            }
        }
        Err(anyhow!("no first result found"))
    }
    fn get_results<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        if let Some(result) = &self
            .get(0)
            .ok_or_else(|| anyhow!("response is empty"))?
            .result
        {
            let json_array = serde_json::Value::Array(result.clone());
            let data: Vec<T> = serde_json::from_value(json_array)?;
            return Ok(data);
        }
        Err(anyhow!("result is empty"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub time: String,
    pub status: String,
    pub result: Option<Vec<serde_json::Value>>,
}

impl Response {
    pub fn get_result<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if let Some(result) = &self.result {
            if let Some(first) = result.first() {
                return Ok(serde_json::from_value(first.clone())?);
            }
        }
        Err(anyhow!("no first result found"))
    }
    pub fn get_results<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        if let Some(result) = &self.result {
            let json_array = serde_json::Value::Array(result.clone());
            let data: Vec<T> = serde_json::from_value(json_array)?;
            return Ok(data);
        }
        Err(anyhow!("result is empty"))
    }
}
