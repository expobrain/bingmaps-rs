use crate::error::{Error, RequestError};
use reqwest::Client as HttpClient;
use reqwest::RequestBuilder;

// use serde_json as json;
use serde_urlencoded as urlencoded;
use std::collections::HashMap;

pub type Params<'a> = HashMap<&'a str, &'a str>;

pub struct Client {
    client: HttpClient,
    key: String, // <-- not to be modified (b.c. Sync)
}

impl Client {
    fn url(path: &str, params: &Params) -> String {
        let query = urlencoded::to_string(params).unwrap_or_else(|_| String::from(""));
        format!("https://dev.virtualearth.net/REST/v1/{}?{}", path, query)
    }

    pub fn new<Str: Into<String>>(key: Str) -> Client {
        Client {
            client: HttpClient::new(),
            key: key.into(),
        }
    }

    pub async fn get<'a, T: serde::de::DeserializeOwned>(
        &'a self,
        path: &'a str,
        params: &mut Params<'a>,
    ) -> Result<T, Error> {
        params.insert("key", &self.key);

        let url = Client::url(path, &params);
        let request = self.client.get(&url);

        send(request).await
    }
}

async fn send<T: serde::de::DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
    let response = request.send().await?;
    let status = response.status();

    if !status.is_success() {
        let mut should_wait = false;

        if let Some(raw) = response.headers().get("X-MS-BM-WS-INFO") {
            for line in raw.to_str().iter() {
                should_wait = *line == "1";
            }
        }

        return Err(Error::from(RequestError {
            http_status: status.as_u16(),
            should_wait,
        }));
    }

    // response.read_to_string(&mut body)?;
    // json::from_str(&body).map_err(Error::from)
    response.json::<T>().await.map_err(Error::from)
}
