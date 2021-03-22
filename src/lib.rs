use log::info;
use std::fmt;
use std::fs;
use std::io::Error;
use std::io::Result as IoResult;
use std::result::Result;

pub struct GistFile {
  path: String,
  id: String,
}
pub struct Gist {
  access_token: String,
  files: Vec<GistFile>,
}
pub struct Config {
  pub log: bool,
  pub s: IoResult<String>, //pub gist: Option<Gist>,
}
impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Config{{log: {}, s: {:?}}}", self.log, self.s)
  }
}
impl Config {
  pub fn new(matches: &clap::ArgMatches) -> Config {
    return Config {
      log: matches.is_present("log"),
      s: fs::read_to_string("/Users/acohen.sinkerrc.json"), //gist: None,
    };
  }
  pub fn read_config_file(&self) -> IoResult<String> {
    if self.log {
      info!("reading config file");
    }
    return fs::read_to_string("/Users/acohen/.sinkerrc.json");
  }
}

pub fn run(matches: clap::ArgMatches) {
  let c = Config::new(&matches);
  println!("{}", c);
}
