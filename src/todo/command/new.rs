use std::result::Result;

use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Create;

#[derive(Debug, Default)]
pub struct Top(pub String);

#[derive(Debug, Default)]
pub struct Scope(pub String);

#[derive(Debug, Default)]
pub struct Id(pub String);

#[derive(Debug, Default)]
pub struct New<T>
    where T: Create + Default
{
    pub create_command: T,
    pub top: Option<Top>,
    pub scope: Option<Scope>,
    pub id: Option<Id>,
    pub name: Option<String>
}

impl<T> New<T>
    where T: Create + Default
{
    pub fn new(create_command: T) -> New<T> {
        New {
            create_command,
            ..New::default()
        }
    }
}

impl<T> Command for New<T>
    where T: Create + Default
{
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        if !value.is_empty() {
            match key.to_lowercase().as_str() {
                "top" | "t" => self.top = Some(Top(value)),
                "scope" | "s" => self.scope = Some(Scope(value)),
                "id" | "i" => self.id = Some(Id(value)),
                _ => self.create_command.set_param(key, value)?,
            }
        } else {
            self.name = Some(key.to_string());
        }
        Ok(())
    }
}