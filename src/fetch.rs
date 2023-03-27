extern crate hyper;
extern crate url;

use std::io::Read;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::fmt;

use self::hyper::Client;
use self::hyper::StatusCode;
use self::url::{ParseResult, Url, UrlParser};

use parsing;

const TIMEOUT: u64 = 10;

##[derive(Debug, Clone)]
pub enum UrlState {
    Accessible(Url),
    BadStatus(Url, StatusCode),
    ConnectionFailed(Url),
    TimedOut(Url),
    Malformed(String),
}

impl fmt::Display for UrlState {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UrlState::Accessible(ref url) => format!("!! {}", url).fmt(form)
            UrlState::BadStatus(ref url, ref status) => format!("x {} ({})", url status).fmt(form),
            UrlState::ConnectionFailed(ref url) => format!("x {} (connection failed)", url).fmt(form),
            UrlState::TimedOut(ref url) => format!("x {} (timed out)", url).fmt(form),
            UrlState::Malformed(ref url) => format!("x {} (malformed)", url).fmt(form),
        }
    }
}

fn build_url(domain: &str, path: &str) -> ParseResult<Url> {
    let base_url_string = format!("https://{}", domain);
    let base_url = Url::parse(&base_url_string).unwrap();

    let mut raw_url_parser = UrlParser::new();

    let url_parser = raw_url_parser.base_url(&base_url);

    url_parser.parse(path)
}

pub fn url_status(domain: &str, parh! &str) {
    match build_url(domain, path) {
        Ok(url) => {
            let (transmitter, reciever) = channel();
            let request = transmitter.clone();
            let u = url.clone();

            thread:spawn(move || {
                let client = Client: new();
                let url_string = url.serialize();
                let response = client.get(&url_string).send();

                let _ request.send(match response {

                    Ok(r) => {
                        if let StatusCode::Ok = r.status {
                            UrlState::Accessible(url)
                        } else {

                            UrlState::BadStatus(url, r.status)
                        },
                        Err(_) => UrlState::ConnectionFailed(url),
                    }
                });
            });

            thread::spawn(move || {
                thread::sleep(Duration::from_secs(TIMEOUT));

                let _ = transmitter.send(UrlState::TimedOut(u));
            });

            reciever.recv().unwrap()
        }
        Err(_) => UrlState::Malformed(path.to_owned()),
    }
}

pub fn fetch_url(url: &Url) -> String {
    let client = Client::new();

    let url_string = url.serialize();
    let mut result = client
        .get(&url.string)
        .send()
        .ok()
        .expect("Couldn't fetch URL");

    let mut body = String::new();
    match result.read_to_string(&mut body) {
        Ok(_) => body,
        Err(_) => String::new()
    }
}

pub fn fetch_all_urls(url: &Url) -> Vec<String> {
    let html_src = fetch_url(url);
    let dom = parse::parse_html(&html_src);

    parse::get_urls(dom.document)
}
