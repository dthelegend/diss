use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::Serialize;

pub trait DataRecorder: Send + Sync {
    fn add_record(&mut self, record: impl Serialize) -> Result<(), Box<dyn Error>>;
}

pub struct CsvDataRecorder<T: Write> {
    wtr: csv::Writer<T>
}

impl CsvDataRecorder<File> {
    pub fn new_from_path<P>(path: P) -> std::io::Result<Self> where P : AsRef<Path> {
        let fd = File::create_new(path)?;

        Ok(Self::new_from_writer(fd))
    }
}

impl <T> CsvDataRecorder<T> where T: Write {
    pub fn new_from_writer(writer : T) -> Self {
        Self::new_with_csv_writer(csv::Writer::from_writer(writer))
    }
    pub fn new_with_csv_writer(wtr: csv::Writer<T>) -> Self {
        Self {
            wtr
        }
    }
}

impl <T> DataRecorder for CsvDataRecorder<T> where T: Write + Send + Sync {
    fn add_record(&mut self, record: impl Serialize) -> Result<(), Box<dyn Error>> {
        self.wtr.serialize(record)?;
        
        Ok(())
    }
}
