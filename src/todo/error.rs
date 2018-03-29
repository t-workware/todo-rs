#[derive(Debug, Fail)]
pub enum TodoError {
    #[fail(display = "unknown command param: {}", param)]
    UnknownCommandParam {
        param: String,
    }
}