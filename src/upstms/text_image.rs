extern crate regex;
extern crate reqwest;

use self::regex::Regex;
use self::reqwest::StatusCode;
use httpc::*;
use std::fs;
use std::{self, str};
use utils;

pub fn gen_html(width: u32, path: &str) {
    let parent_path = utils::get_parent_path(path);
    let tmp_file = parent_path.to_owned() + "/.tim_tmp";
    fs::copy(path, &tmp_file).unwrap();

    let form = reqwest::multipart::Form::new()
        .text("width", width.to_string())
        .text("textcolor", "BLACK")
        .text("bgcolor", "WHITE")
        .text("invert", "0")
        .text("contrast", "0")
        .file("image", &tmp_file)
        .unwrap();
    fs::remove_file(&tmp_file).unwrap();

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
        let file_name = utils::get_filename(path).to_owned();
        utils::save_to(html.as_bytes(), &(file_name + ".html"));
    } else {
        println!("结果解析错误。您可以手动进行排查：https://www.text-image.com");
        std::process::exit(1);
    }
}
