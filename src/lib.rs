#![crate_name = "sonar"]

pub mod args;
mod searcher;
mod walker;

pub fn run(args: args::Args) {
    let paths = walker.walk(args.paths, args.recursive);
    let lines = Vec::new();

    for path in paths {
        match searcher::search(args.get_pattern(), path) {
            Ok(line) => {
                lines.add(line);
            }
            Err(e) => {}
        }
    }

    lines
}
