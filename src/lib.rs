use log::info;
use std::fmt;
use std::fs;
use std::io::Result as IoResult;
use std::io::{Error, ErrorKind};
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
struct CustomError(String);

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Config{{log: {}, s: {:?}}}", self.log, self.s)
  }
}
impl Config {
  pub fn new(matches: &clap::ArgMatches) -> Config {
    return Config {
      log: matches.is_present("log"),
      s: fs::read_to_string("/Users/acohen/.sinkrrc.json").map_err(|err| {
        Error::new(
          ErrorKind::InvalidInput,
          format!("not able to read config file: {}", err),
        )
      }), //gist: None,
    };
  }
}

pub fn run(matches: clap::ArgMatches) {
  let c = Config::new(&matches);
  println!("{}", c);
}
