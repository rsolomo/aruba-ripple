extern crate hyper;
extern crate url;

use hyper::client::RedirectPolicy;
use hyper::client::response::Response;
use hyper::header::{Cookie, ContentType, SetCookie};

#[derive(Debug)]
pub struct Client {
    cookie: Cookie,
    origin: String,
    client: hyper::Client,
}

impl Client {
    pub fn new<S: Into<String>>(origin: S) -> Self {
        let mut hyper_client = hyper::Client::new();
        hyper_client.set_redirect_policy(RedirectPolicy::FollowNone);

        Client {
            origin: origin.into(),
            cookie: Cookie(Vec::new()),
            client: hyper_client,
        }
    }

    pub fn with_client<S: Into<String>>(origin: S, mut hyper_client: hyper::Client) -> Self {
        hyper_client.set_redirect_policy(RedirectPolicy::FollowNone);

        Client {
            origin: origin.into(),
            cookie: Cookie(Vec::new()),
            client: hyper_client,
        }
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<Response, ()> {
        let body = url::form_urlencoded::serialize(vec![("credential_0", username),
                                                        ("credential_1", password),
                                                        ("destination", "/")]);
        let s = format!("{}/LOGIN", self.origin);
        let res_result = self.client
                             .post(&s)
                             .body(&body)
                             .header(ContentType::form_url_encoded())
                             .send();

        let res = match res_result {
            Ok(x) => x,
            Err(_) => {
                return Err(());
            }
        };

        match res.headers.get::<SetCookie>() {
            Some(header) => self.cookie.0 = header.0.to_owned(),
            None => {
                return Err(());
            }
        }

        Ok(res)
    }

    pub fn get(&self, path: &str, query: Option<&str>) -> Result<Response, ()> {
        let s = match query {
            Some(query) => format!("{}/{}?{}", self.origin, path, query),
            None => format!("{}/{}", self.origin, path)
        };
        let res_result = self.client
                             .get(&s)
                             .header(self.cookie.clone())
                             .send();

        Ok(match res_result {
            Ok(x) => x,
            Err(_) => {
                return Err(());
            }
        })
    }
}
