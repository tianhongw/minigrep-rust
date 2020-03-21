use minigrep::config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("faild to parse config, error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
