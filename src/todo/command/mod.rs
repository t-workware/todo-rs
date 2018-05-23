pub mod list;
pub mod new;
pub mod store;

pub use self::list::*;
pub use self::new::*;

use todo::error::TodoError;

pub trait Command {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError>;
    fn default_param_key(&self) -> &str;
    fn exec(&mut self);
}
