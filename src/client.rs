use hyper;
use hyper::client::RedirectPolicy;
use hyper::client::response::Response;
use hyper::header::{Cookie, ContentType, SetCookie};
use url::form_urlencoded;
use error::Error;

/// Airwave API Client
#[derive(Debug)]
pub struct Client {
    cookie: Cookie,
    origin: String,
    client: hyper::Client,
}

impl Client {
    /// Creates a new API client.
    pub fn new<S: Into<String>>(origin: S) -> Self {
        let mut hyper_client = hyper::Client::new();
        hyper_client.set_redirect_policy(RedirectPolicy::FollowNone);

        Client {
            origin: origin.into(),
            cookie: Cookie(Vec::new()),
            client: hyper_client,
        }
    }

    /// Creates a new API client, with a provided Hyper client.
    /// This is useful for having the option of changing settings such as timeouts.
    pub fn with_client<S: Into<String>>(origin: S, mut hyper_client: hyper::Client) -> Self {
        hyper_client.set_redirect_policy(RedirectPolicy::FollowNone);

        Client {
            origin: origin.into(),
            cookie: Cookie(Vec::new()),
            client: hyper_client,
        }
    }

    /// Logs into the Airwave API.
    pub fn login(&mut self, username: &str, password: &str) -> Result<Response, Error> {
        let body = form_urlencoded::serialize(vec![("credential_0", username),
                                                   ("credential_1", password),
                                                   ("destination", "/")]);
        let s = format!("{}/LOGIN", self.origin);
        let res = match self.client
                            .post(&s)
                            .body(&body)
                            .header(ContentType::form_url_encoded())
                            .send() {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::Hyper(e));
            }
        };
        match res.headers.get::<SetCookie>() {
            Some(header) => self.cookie.0 = header.0.to_owned(),
            None => {
                return Err(Error::Authentication);
            }
        }
        Ok(res)
    }

    /// Runs a get request against the Airwave API.
    pub fn get(&self, path: &str, query: Option<&str>) -> Result<Response, Error> {
        let s = match query {
            Some(query) => format!("{}/{}?{}", self.origin, path, query),
            None => format!("{}/{}", self.origin, path),
        };
        match self.client
                  .get(&s)
                  .header(self.cookie.clone())
                  .send() {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::Hyper(e)),
        }
    }
}
