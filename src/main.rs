mod file_info;
mod song_info;
mod xml_parser;

use crate::file_info::FileInfo;
use std::env;
use std::error::Error;

fn main() {
    let songs = xml_parser::parse_library(create_file_info());
    println!("found {} songs", songs.len());
}

fn create_file_info() -> FileInfo {
    FileInfo::new(env::args().collect())
}

