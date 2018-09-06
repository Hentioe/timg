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