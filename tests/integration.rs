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
    let login = client.login(&c.username, &c.password).unwrap();
    assert!(login.status.is_redirection());
}

#[test]
fn get() {
    let c = credentials();
    let mut client = Client::new(c.origin);
    client.login(&c.username, &c.password).unwrap();
    let mut stats = client.get("amp_stats.xml", None).unwrap();
    let mut xml = String::new();
    stats.read_to_string(&mut xml).unwrap();
    println!("{}", xml);
    assert!(xml.contains("<amp:amp_stats"));
}