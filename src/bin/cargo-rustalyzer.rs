extern crate clap;
extern crate rustalyzer;

use clap::App;
use rustalyzer::config;

fn main() {
    let subcommand = config::set_args(App::new("Rustalyzer"));

    let matches = App::new("Cargo Wrapper").bin_name("cargo").subcommand(subcommand).get_matches();

    // Start logger
    // Do stuff
}
