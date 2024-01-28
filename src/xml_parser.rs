use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use const_format::concatcp;
use crate::file_info::FileInfo;
use crate::song_info::SongInfo;

const SONG_PATH: &str = "plist/dict/dict/dict";
const KEY_PATH: &str = concatcp!(SONG_PATH, "/key");

pub fn parse_library(file_info: FileInfo) -> Vec<SongInfo> {
    let mut reader = Reader::from_file(file_info.library()).unwrap();
    let mut songs: Vec<SongInfo> = Vec::new();
    let mut path = Vec::new();
    let mut buffer = Vec::new();
    let mut song_found = false;
    let mut song_info: HashMap<String, String> = HashMap::new();
    let mut key= String::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Err(error) => break eprintln!("{}", error),
            Ok(Event::Eof) => break,
            Ok(Event::Start(node)) => {
                path.push(String::from_utf8(node.name().0.to_vec()).unwrap());
                if get_path(&path) == SONG_PATH {
                    song_found = true;
                }
            }
            Ok(Event::End(_)) => {
                if song_found && !get_path(&path).contains(SONG_PATH) {
                    song_found = false;
                    let song: Option<SongInfo> = SongInfo::new(&song_info);
                    if (song.is_some())
                    {
                        songs.push(song.unwrap());
                    }
                    song_info.clear();
                }
                path.pop();
            }
            Ok(Event::Text(e)) => {
                if !song_found {
                    continue;
                }

                let text: String = e.unescape().unwrap().into_owned();
                if text.trim().is_empty() {
                    continue;
                }

                if get_path(&path) == KEY_PATH {
                    key = text.clone();
                } else {
                    song_info.insert(key.clone(), text.clone());
                }
            }
            Ok(_) => {
                // ignored
            },
        }

        buffer.clear();
    }

    songs
}

fn get_path(path: &Vec<String>) -> String {
    let mut full_path = String::new();
    for p in path {
        full_path.push_str(&p);
        full_path.push_str("/");
    }

    full_path.truncate( full_path.len() - 1);
    full_path
}