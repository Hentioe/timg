extern crate timg;

use timg::upstms::Upstream;
use timg::{utils,httpc, upstms,cli};

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

    let path = if let Some(i) = path.find("http") {
        if i == 0 {
            let filename = utils::get_file_format_from_url(path).unwrap();
            httpc::download(path, &filename).unwrap();
            ("./".to_owned() + &filename).to_string()
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    };

    match output {
        "html" => {
            if let Ok(html_file) =
                upstms::local_conv::LocalImageConv::new().gen_html(&path, width, scale)
            {
                utils::open_in_broswer(&html_file);
            } else {
                panic!("Failed to generate html");
            }
        }
        "none" => upstms::local_conv::LocalImageConv::new().print(&path, width),
        _ => panic!("Unknown output target: {}", output),
    }
}

