#[derive(Debug, Fail)]
pub enum TodoError {
    #[fail(display = "unknown command `{}`", name)]
    UnknownCommand { name: String },

    #[fail(display = "unknown command param `{}`", param)]
    UnknownCommandParam { param: String },

    #[fail(display = "key `{}` is not found", key)]
    KeyNotFound { key: String },

    #[fail(display = "alias `{}` already exists for key `{}`", alias, key)]
    AliasAlreadyExists { alias: String, key: String },

    #[fail(display = "file is not specified")]
    FileNotSpecified,
}
