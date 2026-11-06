use std::{
    io::prelude::*,
    io::{self, BufReader}, 
    fs::{self, File}
};

pub struct FileSystem {
    /// Default param for filesystem Ferrum
    present_dir: String,
    files: Vec<String>,
    
    // Difficult part, it's about orchestrate strokes to Buffer in Manager
    pub strokes_present_files: Vec<String>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            present_dir: "?".to_string(),
            files: Vec::new(),
            strokes_present_files: Vec::new(),
        }
    }

    pub fn set_dir(&mut self, path_arg: String) {
        self.present_dir = path_arg;
    }

    pub fn read_dir(&mut self) -> io::Result<()> {
        for entry in fs::read_dir(&self.present_dir)? {
            let dir = entry?;

            self.files.push(dir.path().to_string_lossy().into_owned()); // I hate relocate PathBuf -> String, it's be a little hardly for me.
            // println!("{:?}", dir.path())
        }
        Ok(())
    }

    pub fn read_file(&mut self, file_name: &str) -> io::Result<()> {
        let f = File::open(file_name)?;
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line?;
            // println!("{line}")
            self.strokes_present_files.push(line);
        }

        Ok(())
    }
}