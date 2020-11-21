// Copyright 2017 Rapidity Networks, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
#[cfg(feature = "with-rustls")]
extern crate hyper_rustls;
#[cfg(feature = "with-openssl")]
extern crate hyper_openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

mod client;
mod common;
mod error;
mod response;

pub use crate::client::Client;
pub use crate::error::{Error, RequestError};
pub use crate::response::Response;
pub mod locations;
