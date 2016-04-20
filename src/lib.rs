#![cfg_attr(test, deny(missing_docs))]

//! # aruba-ripple
//!
//! This client helps with usage of the Aruba Airwave API.
//! Refer to the Airwave API documentation for details on
//! endpoints and the XML schema.
//!
//! ## Example:
//!
//! ```norun
//! extern crate aruba_ripple;
//!
//! use aruba_ripple::Client;
//!
//! let mut client = Client::new("https://aruba.example.com");
//! client.login("username", "password").unwrap();
//! let mut stats = client.get("amp_stats.xml", None).unwrap();
//! let mut xml = String::new();
//! stats.read_to_string(&mut xml).unwrap();
//! // you probably want to use an XML parser at this point
//! assert!(xml.contains("amp:amp_stats"));
//! ```
//!

extern crate hyper;
extern crate url;

mod error;
mod client;

pub use error::Error;
pub use client::Client;
pub use hyper::client::Response;
