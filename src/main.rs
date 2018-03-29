extern crate clap;
extern crate config;
#[macro_use]
extern crate failure;

use clap::{Arg, App, SubCommand};
use types::{Str, OsStrX};
use cmd::Cmd;
use todo::command::{Command, New};
use todo::command::store::fs::Create;

mod todo;
mod types;
mod cmd;

const VERSION: Str = "0.1.0";   // Related with `version` value in Cargo.toml
const PARAMS_ARG_NAME: Str = "params";
const PARAM_SEPARATOR: u8 = b':';

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
            .arg(Arg::with_name(PARAMS_ARG_NAME)
                .multiple(true)
            )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches(Cmd::NEW.name) {
        let mut new_command = New::new(Create { ext: None });
        if let Some(params_arg) = matches.args.get(PARAMS_ARG_NAME) {
            for param in params_arg.vals.iter() {
                let (key, value) = param.split_at_byte(PARAM_SEPARATOR);
                new_command.set_param(key.as_str(), value.as_str().to_string()).unwrap();
                println!("param: {:?}, ({:?}, {:?})", param, key, value);
            }
        }
        println!("command: {:?}", new_command);
    }
}