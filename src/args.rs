#[derive(Debug)]
pub struct Args {
    pattern: String,
    paths: Vec<String>,
    ignore_case: bool,
    pub recursive: bool,
    pub regex: bool,
}

impl Args {
    pub fn build(pattern: String,
                 paths: Vec<String>,
                 ignore_case: bool,
                 recursive: bool,
                 regex: bool)
                 -> Args {
        Args {
            pattern: pattern,
            paths: paths,
            ignore_case: ignore_case,
            recursive: recursive,
            regex: regex,
        }
    }

    pub fn get_pattern(&self) -> String {
        let val = self.pattern.clone();
        match self.ignore_case {
            true => val.to_lowercase(),
            false => val,
        }
    }

    pub fn get_paths(&self) -> Vec<String> {
        self.paths.clone()
    }
}
