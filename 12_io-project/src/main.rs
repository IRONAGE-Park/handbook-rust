use std::env;

use io_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = io_project::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
