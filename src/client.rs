use crate::error::{Error, RequestError};
use hyper::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::net::HttpsConnector;

use serde_json as json;
use serde_urlencoded as urlencoded;
use std::collections::HashMap;
use std::io::Read;

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

    #[cfg(feature = "with-rustls")]
    pub fn new<Str: Into<String>>(key: Str) -> Client {
        use hyper_rustls::TlsClient;

        let tls = TlsClient::new();
        let connector = HttpsConnector::new(tls);
        let client = HttpClient::with_connector(connector);
        Client{client, key: key.into()}
    }

    #[cfg(feature = "with-openssl")]
    pub fn new<Str: Into<String>>(key: Str) -> Client {
        use hyper_openssl::OpensslClient;

        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = HttpClient::with_connector(connector);
        Client{client: client, key: key.into()}
    }

    pub fn get<'a, T: serde::de::DeserializeOwned>(&'a self, path: &'a str, params: &mut Params<'a>) -> Result<T, Error> {
        params.insert("key", &self.key);
        let url = Client::url(path, &params);
        let request = self.client.get(&url);
        send(request)
    }
}

fn send<T: serde::de::DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;
    let status = response.status_raw().0;
    match status {
        200..=299 => {}
        _ => {
            let mut should_wait = false;
            if let Some(raw) = response.headers.get_raw("X-MS-BM-WS-INFO") {
                for line in raw.iter() {
                    should_wait = line == b"1";
                }
            }
            return Err(Error::from(RequestError {
                http_status: status,
                should_wait,
            }));
        }
    }

    json::from_str(&body).map_err(Error::from)
}
