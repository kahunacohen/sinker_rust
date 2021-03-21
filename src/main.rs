use clap::{load_yaml, App};
use std::fmt;

struct Config {
    pub verbose: bool,
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config{{display: {}}}", self.verbose)
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let conf = Config {
        verbose: matches.is_present("verbose"),
    };
    print!("{}", conf)
}
