pub mod list;
pub mod new;
pub mod store;

pub use self::list::*;
pub use self::new::*;

use todo::error::TodoError;
use todo::issue::{Content, Issue};

pub trait Command {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError>;
    fn default_param_key(&self) -> &str;
    fn exec(&mut self);
}

pub trait IssueCommand: Command {
    fn init_from<T: Content>(&mut self, issue: &Issue<T>);
}