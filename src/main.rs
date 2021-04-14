use clap::{load_yaml, App};
use sinker::run;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from(yaml).get_matches();
    run(cli);
}
