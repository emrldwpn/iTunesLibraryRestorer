use std::collections::HashMap;
use mp3_metadata::{AudioTag, MP3Metadata};
use crate::structs::song_info::SongInfo;

pub fn get_song_info<'a>(mp3_metadata: &MP3Metadata, artist_map: &'a HashMap<String, HashMap<String, SongInfo>>) -> Option<&'a SongInfo> {
    if mp3_metadata.tag.is_none() {
        return None
    }

    let metadata: &AudioTag = mp3_metadata.tag.as_ref().unwrap();
    let artist: String = String::from(metadata.artist.trim_matches(char::from(0)));
    let song_title: String = String::from(metadata.title.trim_matches(char::from(0)));
    let song_map: Option<&HashMap<String, SongInfo>> = artist_map.get(&artist);
    if song_map.is_none() {
        return None
    }

    song_map.unwrap().get(&song_title)
}