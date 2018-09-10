extern crate timg;

use timg::img::*;

#[test]
fn gen_grayscale_img() {
    gen_grayscale("./res/img.jpg").save("test.jpg").unwrap();
}

#[test]
fn split_block_avg_val_img() {
    let img = gen_grayscale("./res/img.jpg");
    split_block_avg_val(img, 2, 4);
}
