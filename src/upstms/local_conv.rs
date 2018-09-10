use limg;
use upstms::{UpResult, Upstream};

pub struct LocalImageConv;

impl LocalImageConv {
    pub fn new() -> LocalImageConv {
        LocalImageConv {}
    }
}

impl Upstream for LocalImageConv {
    fn gen_text(&self, path: &str, width: u32) -> UpResult {
        let img = limg::gen_grayscale(path);
        Ok(limg::split_block(img, width))
    }
}
