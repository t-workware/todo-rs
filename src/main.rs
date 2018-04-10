extern crate clap;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use clap::{Arg, App, SubCommand};
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
            )
        )
        .arg(Arg::with_name(Cmd::NEW.name)
            .short(Cmd::NEW.short)
            .long(Cmd::NEW.name)
            .help(Cmd::NEW.desc)
            .multiple(true)
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches(Cmd::NEW.name) {
        let mut cmd_new = New::new(
            Create::default().setup(&settings)
        ).setup(&settings);

        if let Some(params_arg) = matches.args.get(PARAMS_ARG_NAME) {
            for param in params_arg.vals.iter() {
                let (key, value) = param.split_at_byte(PARAM_SEPARATOR);
                cmd_new.set_param(key.as_str(), value.as_str().to_string()).unwrap();
                println!("param: {:?}, ({:?}, {:?})", param, key, value);
            }
        }
        println!("command: {:?}", cmd_new);
        cmd_new.exec();
    }
}