use std::{array, io::{Error, Read, Take, Write}, iter::Map, mem::transmute};

pub trait Krate<T> {
    fn read(&self) -> Result<T, Error>;
    fn write(&self, v: T) -> Result<(), Error>;
}
pub struct KrateLoader<T> {
    pub load: fn() -> Result<T, Error>,
    pub save: fn(T) -> Result<(), Error>,
}

impl <T> Krate<T> for KrateLoader<T> {
    fn read(&self) -> Result<T, Error> {
        (self.load)()
    }
    fn write(&self, v: T) -> Result<(), Error> {
       (self.save)(v)
    }
}


// The extension trait offering object-unsafe methods
trait KrateExt<T>: Krate<T> {
    fn map<B>(&self, to: fn(T) -> B, from: fn(B) -> T) -> impl Krate<B>;
}

// A blanket impl
// impl<T,I> KrateExt<T> for I where I: Krate<T> {
//     fn map<B>(&self, to: fn(T) -> B, from: fn(B) -> T) -> impl Krate<B> {
//         let krate = self;
//         let loader = || -> Result<B, Error> {
//             let t = krate.read().unwrap();
//             let b =  to(t);
//             return Result::Err(Error::new(std::io::ErrorKind::TooManyLinks, "error"));
//         };
//         let saver = |b| -> Result<(), Error> {
//             let b = from(b);
//             let t = self.write(b).unwrap();
//             return Result::Ok(t);
//         };
//         let l: KrateLoader<B> = KrateLoader {
//             load: loader,
//             save: saver
//         };
//         return l;
//     }
// }
