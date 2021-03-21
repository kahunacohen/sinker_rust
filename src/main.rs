use clap::{load_yaml, App};
use std::fmt;

struct Config {
    pub log: bool,
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config{{log: {}}}", self.log)
    }
}
impl Config {
    fn new(matches: &clap::ArgMatches) -> Config {
        Config {
            log: matches.is_present("log"),
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let conf = Config::new(&matches);

    print!("{}", conf)
}
