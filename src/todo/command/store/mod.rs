pub mod fs;
pub mod mongo;

use todo::command::Command;

pub trait Create: Command {}