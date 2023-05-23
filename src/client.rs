use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use std::time::Duration;

pub struct Get<'t> {
    pub url: &'t String,
    pub timeout: Duration,
}

impl<'t> Get<'t> {
    pub fn fetch(&self) -> Option<Response> {
        let client: Client = reqwest::blocking::Client::new();
        let mut result: Option<Response> = None;

        if let Ok(res) = client.get(self.url).timeout(self.timeout).send() {
            match res.status() {
                StatusCode::OK => {
                    result = Some(res);
                }
                _ => println!("something went wrong, status code isn't 200 OK"),
            }
        }
        result
    }
}
