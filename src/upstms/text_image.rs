extern crate regex;
extern crate reqwest;

use self::regex::Regex;
use self::reqwest::StatusCode;
use upstms::{UpResult, Upstream};
use httpc::*;
use std::fs;
use std::result::Result::{Err, Ok};
use std::str;
use utils;

pub struct TextImageCom;

impl TextImageCom {
    pub fn new() -> TextImageCom {
        TextImageCom {}
    }
}

impl Upstream for TextImageCom {
    fn gen_text(&self, path: &str, width: u32) -> UpResult {
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
            return Err("可能是对方服务器发生了点故障。您可以手动进行排查：https://www.text-image.com");
        }
        let body = resp.text().unwrap();
        if let Ok(body) = str::from_utf8(body.as_bytes()) {
            let re = Regex::new(r"<pre>([\s\S]*)\n</pre>").unwrap();
            let cap = re.captures(body).unwrap();
            let html = &cap[1];
            Ok(html.to_string())
        } else {
            Err("响应结果错误。您可以手动进行排查：https://www.text-image.com")
        }
    }
}
