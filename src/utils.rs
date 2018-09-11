extern crate regex;

use self::regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::result::Result;

pub fn get_filename(path: &str) -> &str {
    let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();
    file_name
}

pub fn get_parent_path(path: &str) -> &str {
    let parent_path = Path::new(path).parent().unwrap().to_str().unwrap();
    parent_path
}

pub fn save_to(bytes: &[u8], file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    file.write_all(bytes).unwrap();
}

pub fn remove_file(path: &str) -> ::std::io::Result<()> {
    ::std::fs::remove_file(path)
}

pub fn open_in_broswer(path: &str) {
    Command::new("xdg-open")
        .args(&[path])
        .output()
        .expect("Failed to open browser");
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
