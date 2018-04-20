extern crate regex;
extern crate clap;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate walkdir;

use clap::{App, Arg, SubCommand};
use types::Str;
use cmd::Cmd;
use settings::Settings;

mod todo;
mod types;
mod cmd;
mod settings;

const VERSION: Str = "0.1.0";   // Related with `version` value in Cargo.toml
const PARAMS_ARG_NAME: Str = "params";
const PARAM_SEPARATOR: u8 = b':';

fn main() {
    let settings = Settings::new().expect("Read settings error");

    let matches = App::new("Todo")
        .version(VERSION)
        .about("The issue tracking console tool")
        .subcommand(SubCommand::with_name(Cmd::NEW.name)
            .about(Cmd::NEW.desc)
            .arg(Arg::with_name(PARAMS_ARG_NAME)
                .multiple(true)
                .required(true)
            )
        )
        .arg(Arg::with_name(Cmd::NEW.name)
            .short(Cmd::NEW.short)
            .long(Cmd::NEW.name)
            .help(Cmd::NEW.desc)
            .takes_value(true)
            .multiple(true)
        )
        .subcommand(SubCommand::with_name(Cmd::LIST.name)
            .about(Cmd::LIST.desc)
            .arg(Arg::with_name(PARAMS_ARG_NAME)
                .multiple(true)
            )
        )
        .arg(Arg::with_name(Cmd::LIST.name)
            .short(Cmd::LIST.short)
            .long(Cmd::LIST.name)
            .help(Cmd::LIST.desc)
            .takes_value(true)
            .default_value(".*")
            .multiple(true)
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches(Cmd::NEW.name) {
        Cmd::NEW.process(matches, PARAMS_ARG_NAME, &settings).unwrap();
    } else if let Some(matches) = matches.subcommand_matches(Cmd::LIST.name) {
        Cmd::LIST.process(matches, PARAMS_ARG_NAME, &settings).unwrap();
    } else {
        if matches.occurrences_of(Cmd::NEW.name) > 0 {
            Cmd::NEW.process(&matches, Cmd::NEW.name, &settings).unwrap();
        }
        if matches.occurrences_of(Cmd::LIST.name) > 0 {
            Cmd::LIST.process(&matches, Cmd::LIST.name, &settings).unwrap();
        }
    }
}