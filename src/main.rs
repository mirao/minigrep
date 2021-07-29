use std::env;
use std::process;

use minigrep::*;

/// How to run:
/// ```bash
/// $ cargo run to poem.txt
/// $ CASE_INSENSITIVE=true cargo run to poem.txt`
/// ```
fn main() {
    // --snip--

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
