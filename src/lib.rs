// use log::info;
use std::error::Error;
// use std::fmt;
// use std::fs;
use std::result::Result;

use std::fs::File;

use std::io::BufReader;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GistFile {
  path: String,
  id: String,
}

#[derive(Deserialize)]
pub struct Gist {
  access_token: String,
  files: Vec<GistFile>,
}

pub struct Config {
  pub log: bool,
  pub gist: Result<Gist, Box<dyn Error>>,
}
struct CustomError(String);

// impl fmt::Display for Config {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "Config{{log: {}, s: {:?}}}", self.log, self.s)
//   }
// }
fn parse_config_file() -> Result<Gist, Box<dyn Error>> {
  // Open the file in read-only mode with buffer.
  // If opening fails, return Result with Error varient.
  let file = File::open("/Users/acohen/.sinkerrc.json")?;
  let reader = BufReader::new(file);
  // Read the JSON contents of the file as an instance of `Gist`. Again, if error, return
  //early and propogate to caller.
  let gist = serde_json::from_reader(reader)?;
  return Ok(gist);
}
impl Config {
  pub fn new(matches: &clap::ArgMatches) -> Config {
    return Config {
      log: matches.is_present("log"),
      gist: parse_config_file(),
    };
  }
}

pub fn run(matches: clap::ArgMatches) {
  Config::new(&matches);
  println!("Ran!");
}

fn add(x: u8, y: u8) -> u8 {
  x + y
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(add(1, 2), 3);
  }
}
