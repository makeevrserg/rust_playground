use std::io::{Error, Read, Write};
use std::fs::File;
use crate::file_path_krate::FilePath;

pub trait Krate {
    fn read(&self) -> Result<String, Error>;
    fn write(&self, v: String) -> Result<(), Error>;
}

