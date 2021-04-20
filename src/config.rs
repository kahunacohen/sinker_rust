use super::errors;
use serde::Deserialize;
use std::fmt;
use std::fs;
use std::io::BufReader;

#[derive(Clone, Deserialize)]
pub struct GistFile {
  pub path: String,
  pub id: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Gist {
  pub access_token: String,
  pub files: Vec<GistFile>,
}
pub struct Config {
  pub log: bool,
  pub gist: Result<Gist, errors::ConfigError>,
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Config{{log: {}}}", self.log,)
  }
}
fn parse_config_file() -> Result<Gist, errors::ConfigError> {
  // Open the file in read-only mode with buffer.
  // If opening fails, return Result with Error varient.
  let file = fs::File::open("./.sinkerrc.json")?;
  let reader = BufReader::new(file);
  // Read the JSON contents of the file as an instance of `Gist`. Again, if error, return
  //early and propogate to caller.
  let conf = serde_json::from_reader(reader)?;
  return Ok(conf);
}
impl Config {
  pub fn new(matches: &clap::ArgMatches) -> Config {
    return Config {
      log: matches.is_present("log"),
      gist: parse_config_file(),
    };
  }
}
