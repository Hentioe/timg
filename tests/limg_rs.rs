extern crate timg;

use timg::limg::*;

#[test]
fn gen_grayscale_img() {
    gen_grayscale("./res/img.jpg").save("test.jpg").unwrap();
}

#[test]
fn split_block_avg_val_img() {
    let img = gen_grayscale("./res/img.jpg");
    let result = split_block(img, 100);
    println!("{}", result);
}
