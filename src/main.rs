use std::env;
use std::process;

use minigrep::*;

/// How to run:
/// ```bash
/// $ cargo run nobody poem.txt
/// $ CASE_INSENSITIVE=true cargo run are poem.txt`
/// ```
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
