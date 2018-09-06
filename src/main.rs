extern crate clap;
extern crate timg;

use clap::{App, Arg};

const DEFAULT_WIDTH: &'static str = "100";

fn main() {
    let matches = App::new("TIMG")
        .version("alpha")
        .about("Image to ASCII")
        .author("Hentioe Cl (绅士喵)")
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .help("Set output ASCII text width")
                .takes_value(true),
        ).arg(
            Arg::with_name("path")
                .help("The image file path")
                .required(true)
                .index(1),
        ).get_matches();
    let path = matches.value_of("path").unwrap();
    let width = matches
        .value_of("width")
        .unwrap_or(DEFAULT_WIDTH)
        .parse::<u32>()
        .unwrap();
    timg::upstms::text_image::gen_html(width, path);
}
