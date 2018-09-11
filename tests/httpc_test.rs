extern crate reqwest;
extern crate timg;

use reqwest::StatusCode;
use std::str;
use timg::httpc::*;
use timg::utils;
#[test]
fn http_get() {
    let resp = get("https://www.text-image.com/convert/").unwrap();

    assert_eq!(StatusCode::Ok, resp.status());
}
#[test]
fn http_post() {
    // 测试 text_image 图片生成原始 POST 代码
    let form = reqwest::multipart::Form::new()
        .text("width", "100")
        .text("textcolor", "BLACK")
        .text("bgcolor", "WHITE")
        .text("invert", "0")
        .text("contrast", "0")
        .file("image", "./res/img.jpg")
        .unwrap();
    let mut resp = post("https://www.text-image.com/convert/pic2ascii.cgi", form).unwrap();
    assert_eq!(StatusCode::Ok, resp.status());
    let body = resp.text().unwrap();
    if let Ok(body) = str::from_utf8(&body.as_bytes()) {
        assert!(body.contains("<!-- IMAGE BEGINS HERE -->"))
    }
}

#[test]
fn download_file() {
    let file_name = download(
        "https://www.baidu.com/img/bd_logo1.png?where=super",
        "./bd_logo1.png",
    ).unwrap();
    assert_eq!("bd_logo1.png", file_name);
    utils::remove_file("./bd_logo1.png").unwrap();

}
