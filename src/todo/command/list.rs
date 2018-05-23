use std::result::Result;
use std::mem;

use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Find;

#[derive(Clone, Debug, Default)]
pub struct List<T>
    where T: Find
{
    pub find: Option<T>,
}

impl<T> List<T>
    where T: Find
{
    pub fn new(find_command: T) -> List<T> {
        List {
            find: Some(find_command),
        }
    }
}

impl<T> Command for List<T>
    where T: Find
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !key.is_empty() {
            match key.to_lowercase().as_str() {
                _ if self.find.is_some() => self.find.as_mut().unwrap().set_param(key, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() })
            }
        } else if let Some(find) = self.find.as_mut() {
            let default_key = find.default_param_key().to_string();
            find.set_param(&default_key, value)?;
        }
        Ok(())
    }

    fn default_param_key(&self) -> &str {
        self.find.as_ref()
            .map(|find| find.default_param_key())
            .expect("Find command not exist")
    }

    fn exec(&mut self) {
        let mut find = mem::replace(&mut self.find, None)
            .expect("Find command not exist");
        find.exec();
        self.find = Some(find);
    }
}