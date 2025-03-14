use crate::krate::Krate;
use std::fs::File;
use std::io::{Error, Read, Write};

struct FilePath {
    path: String,
}

pub fn file_krate(path: String) -> impl Krate<String> {
    return FilePath {
        path: path.to_string(),
    };
}



impl Krate<String> for FilePath {
    fn read(&self) -> Result<String, Error> {
        let file = File::open(&self.path);
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
    }

    fn write(&self, v: String) -> Result<(), Error> {
        let file = File::create(&self.path);
        match file {
            Ok(mut f) => f.write_all(v.as_ref()),
            Err(error) => Result::Err(error),
        }
    }
}
