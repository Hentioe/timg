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
    let mut resp = match reqwest::Client::new()
        .get(url)
        .header(UserAgent::new(DEFAULT_USER_AGENT))
        .send()
    {
        Ok(resp) => resp,
        Err(e) => return Err(format!("发起下载请求时候发生错误: {:?}", e)),
    };
    if let Ok(_) = File::create(path).and_then(|mut output_file| io::copy(&mut resp, &mut output_file))
    {
        if let Some(file_name) = utils::get_filename(path) {
            Ok(file_name.to_string())
        } else {
            Err("没有正确获取到下载后的文件名".to_string())
        }
    } else {
        Err("下载文件到本地失败".to_string())
    }
}
