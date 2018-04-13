#[derive(Debug, Fail)]
pub enum TodoError {
    #[fail(display = "unknown command `{}`", name)]
    UnknownCommand {
        name: String,
    },
    #[fail(display = "unknown command param `{}`", param)]
    UnknownCommandParam {
        param: String,
    },
    #[fail(display = "file is not specified")]
    FileNotSpecified,
}