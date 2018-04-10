pub mod new;
pub mod store;

pub use self::new::*;

use todo::error::TodoError;

pub trait Command: Clone + Default {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError>;
    fn exec(&mut self);
}