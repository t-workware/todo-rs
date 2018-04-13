use std::result::Result;
use std::mem;

use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Create;

#[derive(Clone, Debug, Default)]
pub struct List<T>
    where T: Create
{
    pub search: Option<T>,
    pub filter: Option<String>,
}

impl<T> List<T>
    where T: Create
{
    pub fn new(create_command: T) -> List<T> {
        List {
            search: Some(create_command),
            filter: None,
        }
    }
}

impl<T> Command for List<T>
    where T: Create
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !key.is_empty() {
            match key.to_lowercase().as_str() {
                "filter" | "f" => self.filter = Some(value),
                _ if self.search.is_some() => self.search.as_mut().unwrap().set_param(key, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() })
            }
        } else {
            self.filter = Some(value);
        }
        Ok(())
    }

    fn exec(&mut self) {
        let mut search = mem::replace(&mut self.search, None)
            .expect("Create command not exist");
//        search.init_from(&self);
        search.exec();
        self.search = Some(search);
    }
}