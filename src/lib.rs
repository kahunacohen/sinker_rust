use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs;
use std::process;
use std::result::Result;

mod config;
mod errors;

struct SyncData<'a> {
    access_token: &'a String,
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
        access_token: access_token,
        file_modified: fs::metadata(f.path)?.modified()?.into(),
    })
}

// run loads the config. If it's ok it gets all the data needed needed for each file, as
// represented by struct SyncData. If successful, it pipes this along to function responsible
// for actually syncing each file.
pub fn run(matches: clap::ArgMatches) {
    // load the config from the file.
    let conf = config::Config::new(&matches);
    match conf.gist {
        Ok(gist) => {
            // If the config is valid, loop through each file and generate sync data.
            for f in gist.files {
                // We have to borrow the access token string because the reference is taken
                // in previous iterations of the loop.
                let y = get_sync_data(&gist.access_token, f, conf.log).unwrap();
                println!("{}", y.file_modified);
                println!("{}", y.access_token);
            }
        }
        Err(why) => {
            // If there's an error in getting config report it and exit.
            eprintln!("{}", why);
            process::exit(1);
        }
    }
}
