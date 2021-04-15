use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt;
use std::fs;
use std::result::Result;

mod config;
mod errors;

struct SyncData {
    // accessToken: String,
    // file_name: String,
    // gist_content: String,
    file_modified: DateTime<Utc>, // gist_last_mod: DateTime<Utc>,
}
fn get_sync_data(
    access_token: &String,
    f: config::GistFile,
    log: bool,
) -> Result<SyncData, Box<dyn Error>> {
    if log {
        println!("getting sync data for {}", f.path);
    }
    Ok(SyncData {
        file_modified: fs::metadata(f.path)?.modified()?.into(),
    })
}

pub fn run(matches: clap::ArgMatches) {
    let conf = config::Config::new(&matches);
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
