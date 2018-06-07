pub mod setup;
pub use self::setup::*;

use std::collections::HashMap;
use std::env;

use config::{Config, Environment, File};
use failure::Error;
use lang::{Str, ToStringsCollect};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Issue {
    pub attrs: HashMap<String, Vec<String>>,
    pub attrs_order: Option<Vec<String>>,
    pub id_attr_key: String,
    pub default_attr_key: String,
}

impl Default for Issue {
    fn default() -> Self {
        let attrs = [
            ("id", &["i"][..]),
            ("priority", &["p", "top", "t"][..]),
            ("scope", &["s"][..]),
            ("name", &["n", "title"][..]),
        ].to_strings_collect();

        Issue {
            attrs,
            attrs_order: None,
            id_attr_key: "id".to_string(),
            default_attr_key: "name".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FsStore {
    pub attrs: HashMap<String, Vec<String>>,
    pub format: String,
    pub capture: String,
    pub find_all: bool,
    pub issues_dir: String,
    pub ext: String,
    pub id_generator: String,
}

impl FsStore {
    const DEFAULT_FORMAT: Str = "{scope:/}{priority:.}{id:.}{name}{.:ext}";
    const DEFAULT_CAPTURE: Str = r"^(?:(?P<scope>.+)/)?(?:(?P<priority>[^\.]+)\.)??(?:(?P<id>[^\.]+)\.)??(?P<name>[^\.]+)?(?:\.(?P<ext>[^\.]+))?$";
}

impl Default for FsStore {
    fn default() -> Self {
        let attrs = [("all", &["a"][..])].to_strings_collect();

        FsStore {
            attrs,
            format: Self::DEFAULT_FORMAT.to_string(),
            capture: Self::DEFAULT_CAPTURE.to_string(),
            find_all: false,
            issues_dir: "issues".to_string(),
            ext: "md".to_string(),
            id_generator: String::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MongoStore {
    uri: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Store {
    pub fs: FsStore,
    pub mongo: MongoStore,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Generator {
    pub sequence: SequenceGenerator,
}

impl Generator {
    const SEQUENCE: Str = "sequence";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequenceGenerator {
    pub required: bool,
    pub file: String,
}

impl Default for SequenceGenerator {
    fn default() -> Self {
        SequenceGenerator {
            required: false,
            file: "todo.seq".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewCommand {
    pub default_attrs: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Command {
    pub new: NewCommand,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub issue: Issue,
    pub store: Store,
    pub command: Command,
    pub generator: Generator,
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let config_file_name = env::var("TODO_CONFIG_FILE_NAME")
            .unwrap_or_else(|_|"todo.toml".to_string());

        let mut config = Config::new();

        config.merge(Config::try_from(&Settings::default())?)?;

        if let Ok(home) = env::var("TODO_HOME") {
            config.merge(
                File::with_name(&format!("{}/{}", home, config_file_name))
                    .required(false)
            )?;
        }

        config.merge(File::with_name(&config_file_name).required(false))?;

        // Add in settings from the environment (with a prefix of TODO)
        // Eg.. `TODO_SET_DEBUG=1 ./target/todo` would set the `debug` key
        config.merge(Environment::with_prefix("TODO_SET"))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
