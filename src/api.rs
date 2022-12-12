// use std::error::Error;

pub use crate::models::*;

use reqwest::Client;
use serde_json::Value;
// use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Api {
    client: Client,
}

impl Default for Api {
    fn default() -> Self {
        Api::new()
    }
}

impl Api {
    pub fn new() -> Api {
        Api {
            client: Client::new(),
        }
    }

    pub async fn query_location(self: &Api, query: &str) -> Result<Vec<LocationData>, String> {
        let addr = format!("https://api.whynotcats.com/search_location?query={}", query);
        log::info!("Querying {}", addr);
        let response = self.client.get(addr).send().await.unwrap();

        if !response.status().is_success() {
            let error_string = format!(
                "Error fetching data for query {} ({})",
                query,
                response.status(),
            );
            Err(error_string)
        } else {
            let body = response.json::<Value>().await.unwrap();
            let mut data: Vec<LocationData> = Vec::new();

            for raw in body.as_array().unwrap() {
                log::info!("Parsing response: {}", raw);
                let location: LocationData = serde_json::from_value(raw.clone()).unwrap();
                data.push(location);
            }
            Ok(data)
        }
    }
}
