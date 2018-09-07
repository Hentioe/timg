extern crate reqwest;

use self::reqwest::header::UserAgent;
use self::reqwest::multipart::Form;
use self::reqwest::Result;
use std;
use std::fs::File;
use std::io;
use utils;

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";

pub fn get(url: &str) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .get(url)
        .header(UserAgent::new(DEFAULT_USER_AGENT))
        .send()
}

pub fn post(url: &str, form: Form) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .post(url)
        .header(UserAgent::new(DEFAULT_USER_AGENT))
        .multipart(form)
        .send()
}

pub fn download(url: &str, path: &str) -> std::result::Result<String, String> {
    let mut resp = reqwest::Client::new()
        .get(url)
        .header(UserAgent::new(DEFAULT_USER_AGENT))
        .send()
        .unwrap();
    let mut output_file = File::create(path).unwrap();
    match io::copy(&mut resp, &mut output_file) {
        Ok(_) => Ok(utils::get_filename(path).to_string()),
        Err(e) => Err(format!("由于某些原因下载失败: {}", e).to_string()),
    }
}
