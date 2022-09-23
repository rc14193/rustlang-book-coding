use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main(){
    let cli_args = env::args().collect::<Vec<String>>();
    println!("{:?}", cli_args);

    let config = Config::new(&cli_args).unwrap_or_else(|err| {
        eprintln!("Problem processing args of error {}", err);
        process::exit(1)
    });

    println!("Searching for query {}", config.query);
    println!("Searching in file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

