use super::village::Village;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io;

pub struct Loader {
    file_path: String,
}

impl Loader {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
        }
    }

    pub fn load(&self) -> Result<Vec<Village>, Box<dyn Error>>{
        let path = PathBuf::from(&self.file_path);
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(File::open(path)?);
        let mut results = Vec::<Village>::new();
        for result in rdr.deserialize() {
            let village: Village = result?;
            results.push(village);
        }
        return Ok(results);
    }
}
