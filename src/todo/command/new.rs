use std::result::Result;
use std::mem;

use todo::error::TodoError;
use todo::command::Command;
use todo::issue::Issue;
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
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if !param.is_empty() {
            match self.issue.attrs.get_key(param.to_lowercase().as_str())
                .map(|key| key.to_string())
            {
                Some(key) => {
                    self.issue.attrs.set_attr(key, value)?;
                },
                _ if self.create.is_some() => self.create.as_mut().unwrap().set_param(param, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: param.to_string() })
            }
        } else {
            self.issue.attrs.set_default_attr(value);
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