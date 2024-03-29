use std::collections::HashMap;
use std::path::Path;

pub struct SongInfo {
    pub artist: String,
    pub name: String,
    pub album: String,
    pub directory: String,
    pub location: String,
    _private: (),
}

impl SongInfo {
    pub fn new(info: &HashMap<String, String>) -> Option<SongInfo> {
        let kind: Option<&String> = info.get("Kind");
        if kind.is_none() {
            return None
        }

        if !kind.unwrap().to_lowercase().contains("audio file") {
            return None
        }

        let artist: Option<&String> = info.get("Artist");
        if artist.is_none() {
            return None
        }

        let name: Option<&String> = info.get("Name");
        if name.is_none() {
            return None
        }

        let album: Option<&String> = info.get("Album");
        if album.is_none() {
            return None
        }

        let location: Option<&String> = info.get("Location");
        if location.is_none() {
            return None
        }

        let location: String = location.unwrap().to_string()
            .replace("file://localhost/", "")
            .replace("%20", " ");

        let file_name: &str  = Path::new(&location).file_name().unwrap().to_str().unwrap();
        let mut directory: String = location.replace(file_name, "");
        if directory.ends_with("/") {
            directory.truncate(directory.len() - 1);
        }

        Some(
            SongInfo {
                artist: artist.unwrap().to_string(),
                name: name.unwrap().to_string(),
                album: album.unwrap().to_string(),
                directory,
                location,
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

    pub fn directory(&self) -> &str {
        &self.directory
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