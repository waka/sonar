use std::fs;
use std::path::Path;

pub fn walk(paths: Vec<String>, recursive: bool) -> Vec<Path> {
    let 
    for path in paths {
    }
}

struct Walker {
    files: Vec<Path>,
}

impl Walker {
    fn new() -> Walker {
        let files = Vec::new();
        Walker { files: files }
    }

    fn collect(&self, path: Path) -> io::Result<Path> {
        if path.is_dir() {
            for entry in try!(fs::read_dir(dir)) {
                let entry = try!(entry);
                let p = entry.path();
                if p.is_dir() {
                    try!(self.collect(p));
                }
                self.files.push(p);
            }
        } else {
            self.files.push(p);
        }
    }
}
