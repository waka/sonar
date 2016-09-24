#![crate_name = "sonar"]

#[derive(Debug)]
pub struct Args {
    pattern: String,
    paths: Vec<String>,
    ignore_case: bool,
    recursive: bool,
    regex: bool,
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
}

pub fn run(args: &Args) {
    println!("run: {:?}", args);
}
