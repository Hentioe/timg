pub mod httpc;
pub mod upstms;

pub mod utils {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn get_filename(path: &str) -> &str {
        let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();
        file_name
    }

    pub fn save_to(bytes: &[u8], file_path: &str) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(bytes).unwrap();
    }
}
