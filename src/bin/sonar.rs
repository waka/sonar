#![cfg(not(test))]

extern crate env_logger;
extern crate getopts;
extern crate sonar;

use getopts::{Matches, Options};

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show this message");
    opts.optflag("v", "version", "show version information");
    opts.optflag("r", "", "search recursively");
    opts
}

fn print_usage(opts: &Options, reason: &str) {
    let reason = format!("{}\nusage: {} [options] <file>...",
                         reason,
                         env::args_os().next().unwrap().to_string_lossy());
    println!("{}", opts.usage(&reason));
}

fn show_version() {
    println!("sonar version: {}", env!("CARGO_PKG_VERSION"));
}

fn execute(opts: &Options) -> {
    let matches = try!(opts.parse(std::env::args().skip(1)));
}

fn main() {
    let _ = env_logger::init();

    let opts = make_options();

    let exit_code = match execute(&opts) {
        Ok(_) => 0,
        Err(e) => {
            print_usage(&opts, &e.to_string());
            1
        }
    };

    std::io::stdout().flush().unwrap();
    std::process::exit(exit_code);
}
