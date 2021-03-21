use std::fmt;

pub struct Config {
  pub log: bool,
}
impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Config{{log: {}}}", self.log)
  }
}
impl Config {
  pub fn new(matches: &clap::ArgMatches) -> Config {
    Config {
      log: matches.is_present("log"),
    }
  }
}

pub fn run(matches: clap::ArgMatches) {
  let conf = Config::new(&matches);
  print!("{}", conf)
}
