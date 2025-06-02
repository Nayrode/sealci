use crate::error::Error;
use serde::de::DeserializeOwned;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get<T>(&self, url: String, token: String) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let http_response = self
            .client
            .get(url)
            .header("User-Agent", "rust-reqwest")
            .header("Authorization", format!("token {}", token))
            .send()
            .await
            .map_err(Error::RequestError)?;

        // Check if the response status is successful
        let validated_http_response = http_response
            .error_for_status()
            .map_err(Error::RequestError)?;

        // Deserialize the JSON response into the specified type
        let serialized_result = validated_http_response
            .json::<T>()
            .await
            .map_err(Error::RequestError)?;

        Ok(serialized_result)
    }
}
