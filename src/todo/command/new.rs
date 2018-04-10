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
    pub id: Option<Id>,
    pub name: Option<String>
}

impl<T> New<T>
    where T: Create
{
    pub fn new(create_command: T) -> New<T> {
        New {
            create: Some(create_command),
            ..New::default()
        }
    }
}

impl<T> Command for New<T>
    where T: Create
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !value.is_empty() {
            match key.to_lowercase().as_str() {
                "top" | "t" => self.top = Some(Top(value)),
                "scope" | "s" => self.scope = Some(Scope(value)),
                "id" | "i" => self.id = Some(Id(value)),
                "name" | "n" => self.name = Some(value),
                _ if self.create.is_some() => self.create.as_mut().unwrap().set_param(key, value)?,
                _ => ()
            }
        } else {
            self.name = Some(key.to_string());
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