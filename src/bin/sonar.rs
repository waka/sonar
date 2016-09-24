#![cfg(not(test))]

extern crate env_logger;
extern crate getopts;
extern crate sonar;

use std::{env, error};
use std::io::{self, Write};
use std::process;
use std::result;
use getopts::{Matches, Options};

type Error = Box<error::Error + Send + Sync>;
type Result<T> = result::Result<T, Error>;

enum Action {
    Help,
    Version,
    Run {
        args: sonar::Args,
    },
}

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show usage");
    opts.optflag("v", "version", "show version information");
    opts.optflag("r", "recursive", "search recursively");
    opts.optflag("i", "ignore-case", "case sensitive");
    opts.optflag("e", "", "regex switch");
    opts
}

fn print_usage(opts: &Options) {
    let reason = "usage: sonar [options] pattern <...path>";
    println!("{}", opts.usage(&reason));
}

fn print_version() {
    println!("sonar version: {}", env!("CARGO_PKG_VERSION"));
}

fn execute() -> Result<i32> {
    let opts = make_options();
    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();

    match try!(handle_action(&matches)) {
        Action::Help => {
            print_usage(&opts);
        }
        Action::Version => {
            print_version();
        }
        Action::Run { args } => {
            sonar::run(&args);
        }
    }

    Ok(0)
}

fn handle_action(matches: &Matches) -> Result<Action> {
    if matches.opt_present("h") {
        return Ok(Action::Help);
    }
    if matches.opt_present("v") {
        return Ok(Action::Version);
    }
    if 2 > matches.free.len() {
        return Err(Error::from(format!("Invalid args")));
    }

    let pattern = matches.free[0].clone();
    let paths: Vec<_> = matches.free
        .iter()
        .skip(1)
        .map(|path| (path.to_string()))
        .collect();
    let args = sonar::Args::build(pattern,
                                  paths,
                                  matches.opt_present("i"),
                                  matches.opt_present("r"),
                                  matches.opt_present("e"));
    Ok(Action::Run { args: args })
}

fn main() {
    let _ = env_logger::init();

    let exit_code = match execute() {
        Ok(code) => code,
        Err(e) => {
            println!("{}", e.to_string());
            1
        }
    };

    // Make sure standard output is flushed before we exit.
    io::stdout().flush().unwrap();

    // Make sure to finish all necessary cleanup before called this.
    process::exit(exit_code);
}
