use limg;
use upstm::{UpResult, Upstream};

pub struct ImageConv;

impl ImageConv {
    pub fn new() -> ImageConv {
        ImageConv {}
    }
}

impl Upstream for ImageConv {
    fn gen_text(&self, path: &str, width: u32) -> UpResult {
        let img = limg::gen_grayscale(path);
        Ok(limg::split_block(img, width))
    }
}
