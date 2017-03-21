extern crate clap;
extern crate rustalyzer;

use clap::App;
use rustalyzer::config;

fn main() {
    let matches = config::set_args(App::new("Rustalyzer")).get_matches();

    // Start logger
    // Do stuff
}
