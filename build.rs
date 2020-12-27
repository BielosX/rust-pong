extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;

fn main() {
    let download_link = "https://www.1001freefonts.com/d/5722/lato.zip";
    let mut bytes = reqwest::blocking::get(download_link).unwrap().bytes().unwrap();
    create_dir_all("data").expect("Unable to create directory");
    let mut out = File::create("data/lato.zip").expect("Unable to create file");
    out.write_all(&mut bytes).unwrap();
}