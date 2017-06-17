extern crate hyper;
extern crate serde_json as json;
use std::error;
use std::fmt;
use std::io;

/// An error encountered when communicating with the Bing Maps API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Bing Maps.
    Bing(RequestError),
    /// A networking error communicating with the Bing Maps server.
    Http(hyper::Error),
    /// An error reading the response body.
    Io(io::Error),
    /// An error converting between wire format and Rust types.
    Conversion(Box<error::Error + Sync + Send>),
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Bing(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}

/// An error reported by Bing Maps in a request's response.
///
/// For more details see https://msdn.microsoft.com/en-us/library/ff701703.aspx.
#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
    /// The HTTP status in the response.
    #[serde(skip_deserializing)]
    pub http_status: u16,

    /// If should_wait is true, the service may normally have a result for this query
    /// but the servers are currently overloaded.  Wait a few seconds and try again.
    #[serde(skip_deserializing)]
    pub should_wait: bool,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RequestError({})", self.http_status)
    }
}

impl error::Error for RequestError {
    fn description(&self) -> &str {
        "error reported by bing maps"
    }
}
