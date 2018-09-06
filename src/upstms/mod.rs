pub mod text_image;

pub mod upstms {

    pub trait Upstream {
        fn gen_text(path: &str, width: u32) -> Result<&'static str, &'static str>;
        fn gen_html(path: &str, width: u32, scale: f32) -> Result<&'static str, &'static str>;
        fn output(path: &str, width: u32) -> Result<&'static str, &'static str>;
    }
}
