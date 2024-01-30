use std::fs;
use std::collections::HashMap;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use mp3_metadata::{Error, MP3Metadata};
use crate::structs::file_info::FileInfo;
use crate::structs::song_info::SongInfo;
use crate::parsers::song_parser;

pub fn restore_files(file_info: &FileInfo, artist_map: &HashMap<String, HashMap<String, SongInfo>>) -> i32 {
    let mut count: i32 = 0;
    let music_dir = fs::read_dir(file_info.music_directory());
    if music_dir.is_err() {
        eprintln!("Invalid music directory!");
        std::process::exit(1);
    }

    let music_dir:ReadDir = music_dir.unwrap();
    for music_file in music_dir {
        let music_file: DirEntry = music_file.unwrap();
        let mp3_metadata: Result<MP3Metadata, Error>  = mp3_metadata::read_from_file(music_file.path());
        if mp3_metadata.is_err() {
            continue;
        }

        let mp3_metadata: MP3Metadata = mp3_metadata.unwrap();
        let song_info: Option<&SongInfo> = song_parser::get_song_info(&mp3_metadata, &artist_map);
        if song_info.is_none() {
            continue;
        }

        let song_info: &SongInfo = song_info.unwrap();
        if Path::new(song_info.location()).exists() {
            continue;
        }

        let success = fs::create_dir_all(song_info.directory());
        if success.is_err() {
            eprintln!("Failed to create directories!");
            continue;
        }

        let success = fs::copy(music_file.path(), song_info.location());
        if success.is_err() {
            eprintln!("Failed to copy file: {}", song_info);
        } else {
            println!("Successfully restored file: {}", song_info);
            count += 1;
        }
    }

    count
}