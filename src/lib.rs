use chrono::{DateTime, Utc};
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
use std::result::Result;

mod config;
mod errors;

#[derive(Debug)]
struct SyncData<'a> {
    access_token: &'a String,
    file_name: OsString,
    // gist_content: String,
    file_modified: DateTime<Utc>, // gist_last_mod: DateTime<Utc>,
}
fn get_sync_data(
    access_token: &String,
    f: config::GistFile,
    log: bool,
) -> Result<SyncData, errors::SyncError> {
    if log {
        println!("getting sync data for {}", f.path)
    }
    let p = Path::new(&f.path)
        .file_name()
        .ok_or(errors::SyncError::Io(IOError::new(
            ErrorKind::Other,
            format!("Illegal file path: \"{}\"", f.path),
        )))?;
    let fname = OsString::from(p);
    Ok(SyncData {
        file_name: fname,
        access_token: access_token,
        file_modified: fs::metadata(f.path)?.modified()?.into(),
    })
}
fn print_files(gist: config::Gist) -> Result<u8, errors::ConfigError> {
    for f in gist.files {
        println!("{:?}", f.path);
    }
    Ok(1)
}

// run loads the config. If it's ok it gets all the data needed needed for each file, as
// represented by struct SyncData. If successful, it pipes this along to function responsible
// for actually syncing each file.
pub fn run(matches: clap::ArgMatches) {
    let conf = config::Config::new(&matches);
    conf.gist.and_then(print_files).unwrap();
    // conf.gist.and_then(collect_sync_data).and_then(sync)

    // match conf.gist {
    //     Ok(gist) => {
    //         // If the config is valid, loop through each file and generate sync data.
    //         for f in gist.files {
    //             // We have to borrow the access token string because the reference is taken
    //             // in previous iterations of the loop.

    //             match get_sync_data(&gist.access_token, f, conf.log) {
    //                 Ok(data) => println!("{:?}", data),
    //                 Err(why) => {
    //                     eprint!("{}", why);
    //                     process::exit(1);
    //                 }
    //             }
    //         }
    //     }
    //     Err(why) => {
    //         // If there's an error in getting config report it and exit.
    //         eprintln!("{}", why);
    //         process::exit(1);
    //     }
    // }
}
