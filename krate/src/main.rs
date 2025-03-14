use file_path_krate::FileKratePath;
use krate::{ Krate, KrateLoader, };
use std::{arch::aarch64::int32x2_t, fmt::Error, io::{Read, Write}};

mod file_path_krate;
mod krate;

fn main() {
    let krate_loader: KrateLoader<String> = KrateLoader {
        load: Box::new(|| {
            Ok("Hello, world!".to_string())
        }),
        save: Box::new(
            |data: String| {
                println!("Saving data: {}", data);
                Ok(())
            }   
        ),
    };
    krate_loader.write("something".to_string());
    println!("{}", krate_loader.read().unwrap());

    let file_krate = FileKratePath::new("asdd.txt");
    file_krate.write("Hello".to_string());
    println!("{}", file_krate.read().unwrap());
}
