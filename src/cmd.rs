use clap::ArgMatches;
use failure::Error;
use settings::{Settings, Setup};
use todo::error::TodoError;
use todo::command::{Command, New, List};
use todo::command::store::fs::Create;
use types::{Str, OsStrX};


pub struct Cmd {
    pub name: Str,
    pub short: Str,
    pub desc: Str
}

macro_rules! commands {
    ($([$const_name:ident] $name:ident, -$short:ident, --$long:ident $desc:tt),*) => {
        impl Cmd {
            $(
            pub const $const_name: Cmd = Cmd {
                name: stringify!($name),
                short: stringify!($short),
                desc: $desc
            };
            )*
        }
    };
}

commands! {
    [NEW] new, -n, --new    "Create new issue",
    [LIST] list, -l, --list "List issues"
}


impl Cmd {
    pub fn process(&self, matches: &ArgMatches, name: &str, settings: &Settings) -> Result<(), Error> {
        if let Some(params_arg) = matches.args.get(name) {
            let mut cmd: Box<Command>;

            if self.name == Cmd::NEW.name {
                cmd = Box::new(New::new(
                    Create::default().setup(settings)
                ).setup(&settings));
            } else if self.name == Cmd::LIST.name {
                cmd = Box::new(List::new(
                    Create::default().setup(settings)
                ).setup(&settings));
            } else {
                return Err(TodoError::UnknownCommand { name: self.name.to_string() }.into());
            }

            for param in params_arg.vals.iter() {
                let (mut key, mut value) = param.split_at_byte(::PARAM_SEPARATOR);
                cmd.set_param(key.as_str(), value.as_str().to_string())?;
                println!("param: {:?}, ({:?}, {:?})", param, key, value);
            }
            cmd.exec();
        }
        Ok(())
    }
}