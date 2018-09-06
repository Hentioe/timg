extern crate regex;
extern crate reqwest;

use self::regex::Regex;
use self::reqwest::StatusCode;
use httpc::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{self, str};

pub fn gen_html(path: &str) {
    let form = reqwest::multipart::Form::new()
        .text("width", "100")
        .text("textcolor", "BLACK")
        .text("bgcolor", "WHITE")
        .text("invert", "0")
        .text("contrast", "0")
        .file("image", path)
        .unwrap();
    let mut resp = post("https://www.text-image.com/convert/pic2ascii.cgi", form).unwrap();
    if resp.status() != StatusCode::Ok {
        println!("可能是对方服务器发生了点故障。您可以手动进行排查：https://www.text-image.com");
        std::process::exit(1);
    }
    let body = resp.text().unwrap();
    if let Ok(body) = str::from_utf8(&body.as_bytes()) {
        let re =
            Regex::new(r"<!-- IMAGE BEGINS HERE -->([\s\S]*)<!-- IMAGE ENDS HERE -->").unwrap();
        let cap = re.captures(body).unwrap();
        let html = &cap[0];
        let file_name = Path::new(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let mut file = File::create(file_name + ".html").unwrap();
        file.write_all(html.as_bytes()).unwrap();
    } else {
        println!("结果解析错误。您可以手动进行排查：https://www.text-image.com");
        std::process::exit(1);
    }
}
