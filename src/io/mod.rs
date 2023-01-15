use std;
use std::{
    fs,
    error::Error,
};

use crate::traits::{EditorIO, IOStatus};

pub struct FileIO {
    path: Option<String>
}

impl FileIO {
    pub fn new(path: Option<String>) -> FileIO {
        FileIO { path }
    }
}

impl EditorIO for FileIO {
    fn load(&mut self) -> Result<String, Box<dyn Error>> {
        match &self.path {
            Some(path) => Ok(fs::read_to_string(&path)?),
            None => Ok(String::new())
        }
    }
    fn save(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        if let Some(path) = &self.path {
           fs::write(&path, content)?
        }
        Ok(())
    }
    fn status(&self) -> IOStatus {
        match &self.path {
            Some(p) => IOStatus::Attached(p.to_string()),
            None => IOStatus::Blank
        }
    }
}