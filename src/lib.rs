pub mod httpc;
pub mod limg;
pub mod upstms;
pub mod cli;

pub mod utils {
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

    pub fn remove_file(path: &str)  -> ::std::io::Result<()> {
        ::std::fs::remove_file(path)
    }

    pub fn open_in_broswer(path: &str) {
        Command::new("xdg-open")
            .args(&[path])
            .output()
            .expect("Open in broswer failed");
    }

    pub fn get_file_format_from_url(url: &str) -> ::std::result::Result<String, String> {
        let re = Regex::new(r"^https?:.+/(.+\.[^\?]+).+?$").unwrap();
        if let Some(filename) = re.captures(url).map(|cap| cap[1].to_string()){
            Ok(filename)
        }else{
            Err("Image format in URL is not recognized, please specify manually".to_string())
        }
    }
}

#[test]
fn get_file_format_from_url() {
    let url = "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png?ad=df=fdsfsdf=sfs&dsf";
    let filename = utils::get_file_format_from_url(url).unwrap();
    assert_eq!("googlelogo_color_272x92dp.png", filename);
}
