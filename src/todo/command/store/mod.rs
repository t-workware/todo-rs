pub mod fs;
pub mod mongo;

use todo::command::Command;
use todo::issue::{Content, Issue};

pub trait Create: Command {
    fn init_from<T: Content>(&mut self, issue: &Issue<T>);
}