extern crate image;

use self::image::imageops::colorops;
use self::image::{ImageBuffer, Luma};
use std::vec::Vec;

pub fn gen_grayscale(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let img = image::open(path).unwrap();
    colorops::grayscale(&img)
}

pub fn split_block(img: ImageBuffer<Luma<u8>, Vec<u8>>, ascii_width: u32) -> String {
    let (width, height) = img.dimensions();
    let block_width = width / ascii_width;
    let block_height = 2 * block_width;
    let res_width = width % block_width;
    let res_height = height % block_width;
    let cols = width / block_width;
    let rows = height / block_height;

    // println!(
    //     "宽:{},长:{},块大小:{}:{},宽余:{},长余:{},列:{},行:{}",
    //     width, height, block_width,block_height, res_width, res_height, cols, rows
    // );
    let mut avg_list: Vec<String> = Vec::new();
    for row in 0..rows + 1 {
        for col in 0..cols + 1 {
            let (init_left, init_top) = (col * block_width, row * block_height);
            let (range_width, range_height) = if col == cols {
                let size = if row == rows {
                    res_height
                } else {
                    block_height
                };
                (res_width, size)
            } else if row == rows {
                let size = if col == cols { res_width } else { block_width };
                (size, res_height)
            } else {
                (block_width, block_height)
            };
            // println!(
            //     "列:{},行:{},坐标:{}+{}:{}+{}",
            //     col + 1,
            //     row + 1,
            //     init_left,
            //     range_width,
            //     init_top,
            //     range_height
            // );
            let avg = each_block_pixel_avg(&img, (init_left, init_top), range_width, range_height);
            avg_list.push(map_ascii(avg));
        }
        avg_list.push("\n".to_string());
    }
    avg_list.join("")
}

fn each_block_pixel_avg(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    point: (u32, u32),
    width: u32,
    height: u32,
) -> u8 {
    let mut block_all: Vec<u8> = Vec::new();
    let (init_left, init_top) = point;
    for left in 0..width {
        for top in 0..height {
            let pixel = img[(init_left + left, init_top + top)];
            block_all.push(pixel.data[0]);
        }
    }
    let sum: usize = block_all.iter().fold(0, |mut sum, &val| {
        sum += val as usize;
        sum
    });
    let count = block_all.len();
    if count == 0 {
        return 255;
    }
    let avg = sum / count;
    (if avg > 255 { 255 } else { avg }) as u8
}

const ASCII_CHARS: [&'static str; 8] = ["M", "N", "#", "/", "h", "s", "`", " "];

fn map_ascii(gray_value: u8) -> String {
    let avg = 255 / ASCII_CHARS.len();
    let index = gray_value as usize / avg;
    let index = if index > 7 { 7 } else { index };
    ASCII_CHARS[index].to_string()
}
