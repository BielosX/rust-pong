extern crate reqwest;
extern crate zip;

use std::env;
use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;
use std::fs::OpenOptions;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let download_link = "https://www.1001freefonts.com/d/5722/lato.zip";
    let mut bytes = reqwest::blocking::get(download_link).unwrap().bytes().unwrap();
    create_dir_all("data").expect("Unable to create directory");
    let mut zip_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data/lato.zip").expect("Unable to create file");
    zip_file.write_all(&mut bytes).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let mut file = archive.by_name("Lato-Black.ttf").expect("Unable to find file in archive");
    let mut out = File::create("data/Lato-Black.ttf").expect("Unable to create ttf file");
    std::io::copy(&mut file, &mut out).unwrap();
    std::fs::copy("data/Lato-Black.ttf", format!("target/{}/Lato-Black.ttf", profile)).unwrap();
}