use rand::rngs::OsRng;

mod alphabet;
#[allow(dead_code)]
mod commandline;
mod generator;

use crate::generator::generate;
use structopt::StructOpt;

fn main() {
    let options = commandline::CommandlineOptions::from_args();

    if options.print_version {

        return;
    }

    let generation_options = commandline::generation_options_for_commandline_options(options);
    let result = generate(generation_options, &mut OsRng);
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    println!("{}",  &result);
    clipboard.set_contents(result).unwrap();
}
