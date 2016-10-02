#![crate_name = "sonar"]

use std::error;
use std::result::Result;

pub mod args;
mod searcher;
mod walker;

type Error = Box<error::Error + Send + Sync>;

pub fn run(args: args::Args) -> Result<Vec<String>, Error> {
    let paths = try!(walker::walk(args.get_paths(), args.recursive));

    let mut results: Vec<Vec<String>> = vec![];

    // TODO try parallel execution
    for path in paths {
        results.push(try!(searcher::search(args.get_pattern(), path)));
    }

    Ok(results.concat())
}
