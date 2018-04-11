pub mod setup;
pub use self::setup::*;

use std::env;
use config::{Config, File, Environment};
use failure::Error;

use types::Str;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FsStore {
    pub dir: String,
    pub format: String,
    pub ext: String,
}

impl Default for FsStore {
    fn default() -> Self {
        FsStore {
            format: "{scope:/}{top:.}{id:.}{name}{.:ext}".to_string(),
            dir: "issues".to_string(),
            ext: "md".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MongoStore {
    uri: Option<String>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Store {
    pub fs: FsStore,
    pub mongo: MongoStore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Id {
    pub generator: String,
}

impl Id {
    const GENERATOR_SEQUENCE: Str = "sequence";
}

impl Default for Id {
    fn default() -> Self {
        Id {
            generator: Id::GENERATOR_SEQUENCE.to_string()
        }
    }
}

//
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub enum IdGenerator {
//    Sequence
//}
//
//impl Default for IdGenerator {
//    fn default() -> Self {
//        IdGenerator::Sequence
//    }
//}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Generator {
    pub sequence: SequenceGenerator
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequenceGenerator {
    pub file: String,
}

impl Default for SequenceGenerator {
    fn default() -> Self {
        SequenceGenerator {
            file: "todo.seq".to_string()
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewCommand {
    pub scope: String,
    pub top: String,
    pub id: Id,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Command {
    pub new: NewCommand,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub store: Store,
    pub command: Command,
    pub generator: Generator,
}


impl Settings {
    pub fn new() -> Result<Self, Error> {
        let config_file_name = env::var("TODO_CONFIG_FILE_NAME")
            .unwrap_or("todo.toml".to_string());

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