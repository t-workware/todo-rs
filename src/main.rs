extern crate clap;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use clap::{App, Arg, ArgMatches, SubCommand};
use types::{Str, OsStrX};
use cmd::Cmd;
use todo::command::{Command, New};
use todo::command::store::fs::Create;
use settings::{Settings, Setup};

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
        .arg(Arg::with_name(Cmd::LIST.name)
            .short(Cmd::LIST.short)
            .long(Cmd::LIST.name)
            .help(Cmd::LIST.desc)
            .takes_value(true)
            .multiple(true)
        )
        .get_matches();

    if matches.is_present(Cmd::NEW.name) {
        cmd_new_process(&matches, Cmd::NEW.name, &settings);
    }

    if let Some(matches) = matches.subcommand_matches(Cmd::NEW.name) {
        cmd_new_process(matches, PARAMS_ARG_NAME, &settings);
    }
}

fn cmd_new_process(matches: &ArgMatches, name: &str, settings: &Settings) {
    if let Some(params_arg) = matches.args.get(name) {
        let mut cmd_new = New::new(
            Create::default().setup(settings)
        ).setup(&settings);

        for param in params_arg.vals.iter() {
            let (mut key, mut value) = param.split_at_byte(PARAM_SEPARATOR);
            cmd_new.set_param(key.as_str(), value.as_str().to_string()).unwrap();
            println!("param: {:?}, ({:?}, {:?})", param, key, value);
        }
        cmd_new.exec();
    }
}