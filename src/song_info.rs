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
            return None
        }

        if !kind.unwrap().to_lowercase().contains("audio file") {
            return None
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

impl std::fmt::Display for SongInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Artist: {} | Song Name: {} | Album: {} | File Location: {}", self.artist, self.name, self.album, self.location)
    }
}