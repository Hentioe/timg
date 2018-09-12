extern crate regex;

use self::regex::Regex;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::option::Option;
use std::path::Path;
use std::process::{self, Command};
use std::result::Result;

pub fn get_filename(path: &str) -> Option<&str> {
    Path::new(path)
        .file_name()
        .and_then(|file_name| file_name.to_str())
}

pub fn get_parent_path(path: &str) -> Option<&str> {
    Path::new(path)
        .parent()
        .and_then(|parent_path| parent_path.to_str())
}

pub fn save_to(bytes: &[u8], file_path: &str) -> Result<(), String> {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("创建文件时出错: {:?}", e)),
    };
    match file.write_all(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("写入文件时出错: {:?}", e)),
    }
}

pub fn remove_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

pub fn open_in_broswer(path: &str) -> io::Result<process::Output> {
    Command::new("xdg-open").args(&[path]).output()
}

lazy_static! {
    static ref RE_URL: Regex = Regex::new(r"^https?:.+/(.+\.[^\?]+).*$").unwrap();
}

pub fn get_file_format_from_url(url: &str) -> Result<String, String> {
    if let Some(filename) = RE_URL.captures(url).map(|cap| cap[1].to_string()) {
        Ok(filename)
    } else {
        Err("Image format in URL is not recognized, please specify manually".to_string())
    }
}
