use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;

#[derive(Clone, Deserialize)]
pub struct GistFile {
  path: String,
  id: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Gist {
  accessToken: String,
  files: Vec<GistFile>,
}
pub struct Config {
  log: bool,
  gist: Result<Gist, Box<dyn Error>>,
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Config{{log: {}}}", self.log,)
  }
}
fn parse_config_file() -> Result<Gist, Box<dyn Error>> {
  // Open the file in read-only mode with buffer.
  // If opening fails, return Result with Error varient.
  let file = File::open("./.sinkerrc.json")?;
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
fn get_sync_data(_access_token: &String, f: GistFile, log: bool) {
  if log {
    println!("getting sync data for {}", f.path);
  }
}
pub fn run(matches: clap::ArgMatches) {
  let conf = Config::new(&matches);
  match conf.gist {
    Ok(gist) => {
      for f in gist.files {
        get_sync_data(&gist.accessToken, f, conf.log);
      }
    }

    Err(why) => println!("{}", why),
  }
}

// fn add(x: u8, y: u8) -> u8 {
//   x + y
// }

// #[cfg(test)]
// mod tests {
//   // Note this useful idiom: importing names from outer (for mod tests) scope.
//   use super::*;

//   #[test]
//   fn test_add() {
//     assert_eq!(add(1, 2), 3);
//   }
// }
