use std::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::result::Result;

type Error = Box<error::Error + Send + Sync>;

pub fn search(pattern: String, path: PathBuf) -> Result<Vec<String>, Error> {
    let mut stream = match File::open(&path) {
        Ok(fstream) => Box::new(fstream) as Box<Read>,
        Err(e) => {
            return Err(Error::from(e.to_string()));
        }
    };

    let mut content = String::new();
    try!(stream.read_to_string(&mut content));

    let mut lines = vec![];
    for line in content.split("\n") {
        match line.find(&pattern[..]) {
            Some(_) => {
                lines.push(line.to_string());
            }
            _ => {}
        }
    }

    Ok(lines)
}
