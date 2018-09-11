extern crate clap;

use self::clap::{App, Arg};

pub fn gen_args_parser<'a, 'b>() -> App<'a, 'b>{
    App::new("TIMG")
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
        )
}