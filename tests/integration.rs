extern crate aruba_ripple;

use aruba_ripple::Client;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Credentials {
    origin: String,
    username: String,
    password: String,
}

fn credentials() -> Credentials {
    let file = File::open("settings.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let origin = lines.next().unwrap().unwrap();
    let username = lines.next().unwrap().unwrap();
    let password = lines.next().unwrap().unwrap();
    Credentials {
        origin: origin,
        username: username,
        password: password,
    }
}

#[test]
fn login() {
    let c = credentials();
    let mut client = Client::new(c.origin);
    let res = client.login(&c.username, &c.password).unwrap();
    assert!(res.status.is_redirection());
}