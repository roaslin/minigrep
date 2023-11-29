use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // print on stderr output instead of stdout
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Different ways of handling errors

    // if let structure

    // let config = if let Err(e) = Config::build(&args) {
    //     eprintln!("Problem parsing arguments: {e}");
    //     process::exit(1);
    // };

    // Valid error handling more verbose with Match

    // let config: Config = match config {
    //     Ok(value) => value,
    //     Err(err) => {
    //         println!("Problem parsing arguments: {err}");
    //         process::exit(1);
    //     }
    // };

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
