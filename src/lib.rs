pub mod httpc;
pub mod upstms;

pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::process::Command;

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

    pub fn open_in_broswer(path: &str) {
        Command::new("xdg-open")
        .args(&[path])
        .output()
        .expect("Open in broswer failed");
    }
}
