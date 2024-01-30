pub struct FileInfo {
    library: String,
    music_directory: String,
    _private: (),
}

impl FileInfo {
    pub fn new(args: Vec<String>) -> FileInfo {
        if args.len() < 3
        {
            eprintln!("Missing required arguments!");
            std::process::exit(1);
        }

        FileInfo {
            library: String::from(&args[1]),
            music_directory: String::from(&args[2]),
            _private: (),
        }
    }

    pub fn library(&self) -> &str {
        &self.library
    }

    pub fn music_directory(&self) -> &str {
        &self.music_directory
    }
}