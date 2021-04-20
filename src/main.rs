use clap::{load_yaml, App};
use sinker::run;

// Our main function offloads to lib::run()
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from(yaml).get_matches();
    run(cli);
}
