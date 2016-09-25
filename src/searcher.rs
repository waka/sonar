use std::error;
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::path::PathBuf;

type Error = Box<error::Error + Send + Sync>;

pub fn search(pattern: String, path: PathBuf) -> Result<bool, Error> {
    let mut stream = match File::open(&path) {
        Ok(fstream) => Box::new(fstream) as Box<Read>,
        Err(e) => {
            return Err(Error::from(e.to_string()));
        }
    };

    let mut content = String::new();
    try!(stream.read_to_string(&mut content));

    let lines = content.split("\n");
    for line in lines {
        match line.find(&pattern[..]) {
            Some(_) => {
                println!("{}", line);
            }
            _ => {}
        }
    }

    Ok(true)
}
