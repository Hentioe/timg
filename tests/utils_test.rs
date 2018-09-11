extern crate timg;

use timg::utils;

#[test]
fn get_filename() {
    let file_name = "img.jpg";
    let path = "./res/".to_owned() + file_name;

    assert_eq!(utils::get_filename(&path), file_name);
}
#[test]
fn get_parent_path() {

    let file_name = "img.jpg";
    let path = "./res/".to_owned() + file_name;

    assert_eq!(utils::get_parent_path(&path), "./res");
}


#[test]
fn get_file_format_from_url() {
    let url = "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png?ad=df=fdsfsdf=sfs&dsf";
    let filename = utils::get_file_format_from_url(url).unwrap();
    assert_eq!("googlelogo_color_272x92dp.png", filename);
}