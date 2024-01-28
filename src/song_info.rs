use std::collections::HashMap;

pub struct SongInfo {
    pub artist: String,
    pub name: String,
    pub album: String,
    pub location: String,
    _private: (),
}

impl SongInfo {
    pub fn new(mut info: &HashMap<String, String>) -> Option<SongInfo> {
        let kind = info.get("Kind");
        if kind.is_none() {
            None
        }

        if !kind.unwrap().to_lowercase().contains("audio file") {
            None
        }

        Some(
            SongInfo {
                artist: info.get("Artist").unwrap().to_string(),
                name: info.get("Name").unwrap().to_string(),
                album: info.get("Album").unwrap().to_string(),
                location: info.get("Location").unwrap().to_string(),
                _private: (),
            }
        )
    }

    pub fn artist(&self) -> &str {
        &self.artist
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn album(&self) -> &str {
        &self.album
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}