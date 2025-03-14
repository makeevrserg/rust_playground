use crate::krate::{Krate, KrateLoader};
use std::fs::File;
use std::io::{Error, Read, Write};

pub struct FileKratePath;


impl FileKratePath {
    pub fn new(path: &'static str) -> impl Krate<String> {
        let load=  move || {
            let file = File::open(&path);
            match file {
                Ok(mut f) => {
                    let mut contents = String::new();
                    let read_result = f.read_to_string(&mut contents);
    
                    match read_result {
                        Ok(_) => Result::Ok(contents),
                        Err(error) => Result::Err(error),
                    }
                }
                Err(error) => Result::Err(error),
            }
        };
        let save =  move |data: String| {
                let file = File::create(&path);
                match file {
                    Ok(mut f) => f.write_all(data.as_ref()),
                    Err(error) => Result::Err(error),
                }
            };

        let krate_loader: KrateLoader<String> = KrateLoader {
            load: Box::new(load),
            save: Box::new(save),
        };
        return krate_loader;
    }
}

