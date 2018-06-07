pub mod fs;
pub mod mongo;

use todo::command::IssueCommand;

pub trait Create: IssueCommand {}
pub trait Find: IssueCommand {}
