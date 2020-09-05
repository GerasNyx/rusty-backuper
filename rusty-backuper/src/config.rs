use std::env;

pub struct Config {
    pub from: String,
    pub to: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        // drop program name
        args.next();

        let from = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get `from` path"),
        };

        let to = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get `to` path"),
        };

        Ok(Config { from, to })
    }
}

