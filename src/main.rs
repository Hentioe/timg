extern crate timg;

use timg::utils::{self, safe_exit};
use timg::{cli, conv, httpc, upstm::Upstream};

const DEFAULT_WIDTH: &'static str = "100";
const DEFAULT_SCALE: &'static str = "1";

fn main() {
    let matches = cli::gen_args_parser().get_matches();
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

    let is_url = if let Some(i) = path.find("http") {
        i == 0
    } else {
        false
    };

    let path = if is_url {
        let filename = utils::get_file_format_from_url(path).unwrap();
        safe_exit(httpc::download(path, &filename));
        ("./".to_owned() + &filename).to_string()
    } else {
        path.to_string()
    };

    match output {
        "html" => match conv::ImageConv::new().gen_html(&path, width, scale) {
            Ok(html_file) => safe_exit(utils::open_in_broswer(&html_file)),
            Err(e) => utils::cause_exit(e),
        },
        "none" => safe_exit(conv::ImageConv::new().print(&path, width)),
        _ => panic!("Unknown output target: {}", output),
    }
}
