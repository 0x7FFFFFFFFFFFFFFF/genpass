use rand::rngs::OsRng;

mod alphabet;
#[allow(dead_code)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod commandline;
mod generator;

use crate::generator::generate;
use structopt::StructOpt;

fn main() {
    let options = commandline::CommandlineOptions::from_args();

    if options.print_version {
        println!(
            "This is {} v{}{}, built on {} by {}.",
            built_info::PKG_NAME,
            built_info::PKG_VERSION,
            built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" ({})", v)),
            built_info::BUILT_TIME_UTC,
            built_info::RUSTC_VERSION
        );
        return;
    }

    let generation_options = commandline::generation_options_for_commandline_options(options);
    let result = generate(generation_options, &mut OsRng);
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    println!("{}",  &result);
    clipboard.set_contents(result).unwrap();
}
