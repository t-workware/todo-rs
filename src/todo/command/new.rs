use std::result::Result;
use std::mem;

use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Create;

#[derive(Clone, Debug, Default)]
pub struct Top(pub String);

#[derive(Clone, Debug, Default)]
pub struct Scope(pub String);

#[derive(Clone, Debug, Default)]
pub struct Id(pub String);

#[derive(Clone, Debug, Default)]
pub struct New<T>
    where T: Create
{
    pub create: Option<T>,
    pub top: Option<Top>,
    pub scope: Option<Scope>,
    pub name: Option<String>,
    pub id: Option<Id>,
}

impl<T> New<T>
    where T: Create
{
    pub fn new(create_command: T) -> New<T> {
        New {
            create: Some(create_command),
            top: None,
            scope: None,
            name: None,
            id: None,
        }
    }
}

impl<T> Command for New<T>
    where T: Create
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !key.is_empty() {
            match key.to_lowercase().as_str() {
                "top" | "t" => self.top = Some(Top(value)),
                "scope" | "s" => self.scope = Some(Scope(value)),
                "id" | "i" => self.id = Some(Id(value)),
                "name" | "n" => self.name = Some(value),
                _ if self.create.is_some() => self.create.as_mut().unwrap().set_param(key, value)?,
                _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() })
            }
        } else {
            self.name = Some(value);
        }
        Ok(())
    }

    fn exec(&mut self) {
        let mut create = mem::replace(&mut self.create, None)
            .expect("Create command not exist");
        create.init_from(&self);
        create.exec();
        self.create = Some(create);
    }
}