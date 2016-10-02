use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub fn walk(paths: Vec<String>, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut walker = Walker::new(recursive);
    for path in paths {
        try!(walker.collect(PathBuf::from(path)));
    }
    Ok(walker.files)
}

struct Walker {
    files: Vec<PathBuf>,
    recursive: bool,
}

impl Walker {
    fn new(recursive: bool) -> Walker {
        Walker {
            files: vec![],
            recursive: recursive,
        }
    }

    fn collect(&mut self, path: PathBuf) -> Result<()> {
        if path.is_dir() {
            for entry in try!(fs::read_dir(path)) {
                let entry = try!(entry);
                let p = entry.path();
                if p.is_dir() {
                    if self.recursive {
                        try!(self.collect(p));
                    }
                } else {
                    self.files.push(p);
                }
            }
        } else {
            self.files.push(path);
        }

        Ok(())
    }
}
