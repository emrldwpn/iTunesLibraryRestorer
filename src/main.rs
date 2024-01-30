mod parsers;
mod structs;
mod services;

use crate::parsers::xml_parser;
use crate::services::file_restorer;
use crate::structs::file_info::FileInfo;
use crate::structs::song_info::SongInfo;
use std::collections::HashMap;
use std::env;

/**
 * iTunesLibraryRestorer parses an iTunes Library XML file and
 * restores the associated mp3 files to the proper file name and location.
 * Two command line arguments are required:
 * 1) Path to Library XML file
 * 2) Path to recovered mp3 files
**/
fn main() {
    let file_info: FileInfo = create_file_info();
    let artist_map: HashMap<String, HashMap<String, SongInfo>> = xml_parser::parse_library(&file_info);
    let restored_count: i32 = file_restorer::restore_files(&file_info, &artist_map);
    println!("Successfully restored {} songs", restored_count);
}

fn create_file_info() -> FileInfo {
    FileInfo::new(env::args().collect())
}