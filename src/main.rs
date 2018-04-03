#[macro_use]
extern crate clap;
extern crate lib;

use clap::{App, AppSettings, SubCommand};

const MIX: &str = "mix";
const SCAN: &str = "scan";

fn main() {
    let matches = App::new("decksterity")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .subcommand(SubCommand::with_name(MIX).about("Start audio engine and DJ decks"))
        .subcommand(SubCommand::with_name(SCAN).about("Populate library with audio tracks"))
        .get_matches();

    match matches.subcommand_name() {
        Some(MIX) => lib::mix(),
        Some(SCAN) => lib::scan(),
        other => panic!("Unknown SubCommand {:?}", other),
    }
}
