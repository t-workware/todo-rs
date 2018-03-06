extern crate clap;
extern crate config;

use clap::{Arg, App, SubCommand};
use types::Str;
use commands::Cmd;

mod types;
mod commands;

// Related with `version` value in Cargo.toml
const VERSION: Str = "0.1.0";

fn main() {
    let matches = App::new("Todo")
        .version(VERSION)
        .about("The issue tracking console tool")
        .arg(Arg::with_name(Cmd::NEW.name)
            .short(Cmd::NEW.short)
            .long(Cmd::NEW.name)
            .help(Cmd::NEW.desc)
        )
        .subcommand(SubCommand::with_name(Cmd::NEW.name)
            .about(Cmd::NEW.desc)
            .arg_from_usage("-l, --list 'lists test values'")
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches(Cmd::NEW.name) {
        if matches.is_present("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}