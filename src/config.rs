use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("query not found"),
        };

        let file_name = match args.next() {
            Some(v) => v,
            None => return Err("file name not found"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            file_name: file_name,
            case_sensitive: case_sensitive,
        })
    }
}
