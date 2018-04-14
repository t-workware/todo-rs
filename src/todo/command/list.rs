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
    pub filter: Option<String>,
}

impl<T> List<T>
    where T: Find
{
    pub fn new(find_command: T) -> List<T> {
        List {
            find: Some(find_command),
            filter: None,
        }
    }
}

impl<T> Command for List<T>
    where T: Find
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !key.is_empty() {
            match key.to_lowercase().as_str() {
                "filter" | "f" => self.filter = Some(value),
                _ if self.find.is_some() => self.find.as_mut().unwrap().set_param(key, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() })
            }
        } else {
            self.filter = Some(value);
        }
        Ok(())
    }

    fn exec(&mut self) {
        let mut find = mem::replace(&mut self.find, None)
            .expect("Find command not exist");
//        search.init_from(&self);
        find.exec();
        self.find = Some(find);
    }
}