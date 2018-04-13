use std::result::Result;
use std::mem;

use todo::error::TodoError;
use todo::command::Command;
use todo::issue::{Issue, Id, Top, Scope};
use todo::command::store::Create;

#[derive(Clone, Debug, Default)]
pub struct New<T>
    where T: Create
{
    pub create: Option<T>,
    pub issue: Issue<String>,
}

impl<T> New<T>
    where T: Create
{
    pub fn new(create_command: T) -> New<T> {
        New {
            create: Some(create_command),
            issue: Issue::<String>::default(),
        }
    }
}

impl<T> Command for New<T>
    where T: Create
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !key.is_empty() {
            match key.to_lowercase().as_str() {
                "top" | "t" => self.issue.top = Some(Top(value)),
                "scope" | "s" => self.issue.scope = Some(Scope(value)),
                "id" | "i" => self.issue.id = Some(Id(value)),
                "name" | "n" => self.issue.name = Some(value),
                _ if self.create.is_some() => self.create.as_mut().unwrap().set_param(key, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() })
            }
        } else {
            self.issue.name = Some(value);
        }
        Ok(())
    }

    fn exec(&mut self) {
        let mut create = mem::replace(&mut self.create, None)
            .expect("Create command not exist");
        create.init_from(&self.issue);
        create.exec();
        self.create = Some(create);
    }
}