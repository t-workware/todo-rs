use clap::ArgMatches;
use failure::Error;
use lang::{OsStrX, Str};
use settings::{Settings, Setup};
use todo::command::store::fs::{Create, Find};
use todo::command::{Command, List, New};
use todo::error::TodoError;
use todo::issue::Issue;

pub struct Cmd {
    pub name: Str,
    pub short: Str,
    pub desc: Str,
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
    pub fn process(
        &self,
        matches: &ArgMatches,
        name: &str,
        settings: &Settings,
    ) -> Result<(), Error> {
        let mut cmd: Box<dyn Command>;
        let issue = Issue::<String>::default().setup(settings);

        if self.name == Cmd::NEW.name {
            cmd = Box::new(
                New {
                    create: Some(Create::default().setup(settings)),
                    issue,
                }.setup(&settings),
            );
        } else if self.name == Cmd::LIST.name {
            cmd = Box::new(
                List {
                    find: Some(Find::default().setup(settings)),
                    issue,
                }.setup(&settings)
            );
        } else {
            return Err(TodoError::UnknownCommand {
                name: self.name.to_string(),
            }.into());
        }

        if let Some(params_arg) = matches.args.get(name) {
            for param in &params_arg.vals {
                let (mut key, mut value) = param.split_at_byte(::PARAM_SEPARATOR);
                cmd.set_param(key.as_str(), value.as_str().to_string())?;
            }
        }
        cmd.exec();
        Ok(())
    }
}
