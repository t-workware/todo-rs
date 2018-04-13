pub mod fs;
pub mod mongo;

use todo::command::{Command, New};

pub trait Create: Command {
    fn init_from(&mut self, new: &New<Self>) where Self: Sized;
}