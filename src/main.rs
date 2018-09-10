extern crate clap;
extern crate timg;

use clap::{App, Arg};
use timg::upstms::Upstream;
use timg::utils;
use timg::{httpc, upstms};

const DEFAULT_WIDTH: &'static str = "100";
const DEFAULT_SCALE: &'static str = "1";

fn main() {
    let matches = App::new("TIMG")
        .version("alpha")
        .about("Image to ASCII")
        .author("Hentioe Cl (绅士喵)")
        .arg(
            Arg::with_name("path")
                .help("The image file path")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .help("Set output ASCII text width")
                .takes_value(true),
        ).arg(
            Arg::with_name("scale")
                .long("scale")
                .short("s")
                .help("Scale html font size, unit: [pixel]")
                .takes_value(true),
        ).arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .help("Output target: html/default: text for terminal")
                .takes_value(true),
        ).get_matches();
    let path = matches.value_of("path").unwrap();
    let width = matches
        .value_of("width")
        .unwrap_or(DEFAULT_WIDTH)
        .parse::<u32>()
        .unwrap_or(DEFAULT_WIDTH.parse::<u32>().unwrap());
    let output = matches.value_of("output").unwrap_or("none");
    let scale = matches
        .value_of("scale")
        .unwrap_or(DEFAULT_SCALE)
        .parse::<u32>()
        .unwrap_or(DEFAULT_SCALE.parse::<u32>().unwrap());

    let path = if let Some(i) = path.find("http") {
        if i == 0 {
            httpc::download(path, "./.timg_download").unwrap();
            "./.timg_download"
        } else {
            path
        }
    } else {
        path
    };

    match output {
        "html" => {
            if let Ok(html_file) =
                upstms::local_conv::LocalImageConv::new().gen_html(path, width, scale)
            {
                utils::open_in_broswer(&html_file);
            } else {
                panic!("Gennerte html failed");
            }
        }
        "none" => upstms::local_conv::LocalImageConv::new().print(path, width),
        _ => panic!("Unknown output target: {}", output),
    }
}
