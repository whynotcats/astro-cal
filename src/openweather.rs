pub use crate::models::*;

use reqwest::{Client, Result};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct OpenWeather {
    pub api_key: String,
    client: Client,
}

impl OpenWeather {
    pub fn new(api_key: &str) -> OpenWeather {
        OpenWeather {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    // pub async fn query_location(&self, query: &str) -> Result<Vec<LocationData>> {
    //     self.query(&format!("direct?q={}", &query)).await
    // }

    fn format_addr(&self, query: &str) -> String {
        let base_http = "https://api.openweathermap.org/geo/1.0/";
        format!("{}{}&limit=5&appid={}", &base_http, &query, self.api_key,)
    }

    async fn query<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        let addr = self.format_addr(query);
        let res = self.client.get(addr).send().await?;

        let data = res.json().await?;
        Ok(data)
    }
}
