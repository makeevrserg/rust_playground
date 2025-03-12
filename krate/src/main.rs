use std::io::{Read, Write};
use file_path_krate::FilePath;
use krate::Krate;

mod krate;
mod file_path_krate;

fn main() {
    let path = FilePath {
        path: String::from("./file.txt"),
    };
    path.write(String::from("Hello world")).unwrap();

    println!("{}", path.read().unwrap().as_str());
}
