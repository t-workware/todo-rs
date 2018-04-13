pub mod new;
pub mod list;
pub mod store;

pub use self::new::*;
pub use self::list::*;

use todo::error::TodoError;

pub trait Command {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError>;
    fn exec(&mut self);
}