use chrono::{DateTime, FixedOffset, ParseResult, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

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
struct CustomError(String);

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config{{log: {}}}", self.log,)
    }
}
fn parse_config_file() -> Result<Gist, Box<dyn Error>> {
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
struct SyncData {
    // accessToken: String,
    // file_name: String,
    // gist_content: String,
    file_modified: DateTime<Utc>, // gist_last_mod: DateTime<Utc>,
}
fn get_sync_data(
    access_token: &String,
    f: GistFile,
    log: bool,
) -> Result<SyncData, Box<dyn Error>> {
    if log {
        println!("getting sync data for {}", f.path);
    }
    Ok(SyncData {
        file_modified: fs::metadata(f.path)?.modified()?.into(),
    })
}
async fn x() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("response: {:#?}", resp);
    Ok(())
}
pub async fn run(matches: clap::ArgMatches) {
    //x().await;
    let conf = Config::new(&matches);
    match conf.gist {
        Ok(gist) => {
            for f in gist.files {
                let y = get_sync_data(&gist.accessToken, f, conf.log);
                println!("{}", y.unwrap().file_modified);
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
