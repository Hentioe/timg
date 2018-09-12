
use utils;

pub type UpResult = Result<String, &'static str>;

pub trait Upstream {
    fn gen_text(&self, path: &str, width: u32) -> UpResult;

    fn gen_html(&self, path: &str, width: u32, scale: u32) -> UpResult {
        match self.gen_text(path, width) {
            Ok(text) => {
                let file_name = utils::get_filename(path).unwrap();
                let html = format!(
                    "<font style=\"font-size:{}px\" color=\"black\">\
                     <pre>{}</pre>\
                     </font>",
                    scale, text
                );
                let output_file = format!("{}.html", file_name);
                utils::save_to(html.as_bytes(), &output_file).unwrap();
                Ok(output_file)
            }
            err => err,
        }
    }

    fn print(&self, path: &str, width: u32) {
        println!("{}", self.gen_text(path, width).unwrap())
    }
}
