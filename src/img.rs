extern crate image;

use self::image::imageops::colorops;
use self::image::{ImageBuffer, Luma};
use std::vec::Vec;

pub fn gen_grayscale(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let img = image::open(path).unwrap();
    colorops::grayscale(&img)
}

pub fn split_block_avg_val(img: ImageBuffer<Luma<u8>, Vec<u8>>, width_size: u32, height_size: u32) -> Vec<u8> {
    let (width, height) = img.dimensions();
    let res_width = width % width_size;
    let res_height = height % height_size;
    let cols = width / width_size;
    let rows = height / height_size;

    // println!(
    //     "宽:{},长:{},块大小:{},宽余:{},长余:{},列:{},行:{}",
    //     width, height, size, res_width, res_height, cols, rows
    // );
    let mut avg_list: Vec<u8> = Vec::new();
    for row in 0..rows + 1 {
        for col in 0..cols + 1 {
            let (init_left, init_top) = (col * width_size, row * height_size);
            let (range_width, range_height) = if col == cols {
                let size = if row == rows { res_height } else { height_size };
                (res_width, size)
            } else if row == rows {
                let size = if col == cols { res_width } else { width_size };
                (size, res_height)
            } else {
                (width_size, height_size)
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
            print!("{}", map_ascii(avg));
            avg_list.push(avg);
        }
        println!("");
    }
    avg_list
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
    let avg = sum / block_all.len();
    (if avg > 255 { 255 } else { avg }) as u8
}

const ASCII_CHARS: [&'static str; 8] = ["M", "N", "#", "/", "h", "s", "`", " "];

fn map_ascii(gray_value: u8) -> &'static str {
    let avg = 255 / ASCII_CHARS.len();
    let index = (gray_value as usize / avg);
    let index = if index > 7 {7} else {index};
    ASCII_CHARS[index]
}
