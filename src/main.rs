extern crate timg;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let path = &args[1];
        if Path::new(&path).exists() {
            timg::upstms::text_image::gen_html(path);
        } else {
            println!("请输入正确的图片文件路径！")
        }
    }
}
