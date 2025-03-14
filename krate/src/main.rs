use crate::file_path_krate::file_krate;
use krate::{ Krate, KrateLoader, };
use std::{arch::aarch64::int32x2_t, io::{Read, Write}};

mod file_path_krate;
mod krate;

fn main() {
    let file_krate = file_krate(String::from("./file.txt"));
    file_krate.write(String::from("Hello world")).unwrap();
    // let transformer: Transformer<String, i32,dyn Krate<String>> = create_transformer(
    //     krate,
    //     |x: String| -> 1,
    //     |x: int| -> {String::from("value")}
    // );

    println!("{}", file_krate.read().unwrap().as_str());
}
