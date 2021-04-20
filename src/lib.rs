use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs;
use std::process;
use std::result::Result;

mod config;
mod errors;

struct SyncData<'a> {
    accessToken: &'a String,
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
        accessToken: access_token,
        file_modified: fs::metadata(f.path)?.modified()?.into(),
    })
}

pub fn run(matches: clap::ArgMatches) {
    let conf = config::Config::new(&matches);
    match conf.gist {
        Ok(gist) => {
            for f in gist.files {
                // We have to borrow the access token string because the reference is taken
                // in previous iterations of the loop.
                let y = get_sync_data(&gist.accessToken, f, conf.log).unwrap();
                println!("{}", y.file_modified);
                println!("{}", y.accessToken);
            }
        }
        Err(why) => {
            eprintln!("{}", why);
            process::exit(1);
        }
    }
}
