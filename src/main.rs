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
            .multiple(true)
        )
        .subcommand(SubCommand::with_name(Cmd::NEW.name)
            .about(Cmd::NEW.desc)
            .arg(Arg::with_name("params")
                .multiple(true)
            )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches(Cmd::NEW.name) {
        if let Some(arg) = matches.args.get("params") {
            for val in arg.vals.iter() {
                println!("val: {:?}", val);
            }
        }
    }
}