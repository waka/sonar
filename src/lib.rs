#![crate_name = "sonar"]

use args::Args;

pub mod args;
mod searcher;

pub fn run(args: Args) {
    println!("run: {:?}", args);
    println!("pattern: {}", args.get_pattern());
    println!("paths: {:?}", args.get_paths());

    // pathをwalk throughして検索していく
    for path in args.get_paths() {
        let _ = searcher::search(args.get_pattern(), path).unwrap();
    }
}
