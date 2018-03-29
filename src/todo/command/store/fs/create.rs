use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Create as CanCreate;

#[derive(Debug, Default)]
pub struct Create {
    pub ext: Option<String>
}

impl Command for Create {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        match key.to_lowercase().as_str() {
            "ext" | "e" => self.ext = Some(value),
            _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() }),
        }
        Ok(())
    }
}

impl CanCreate for Create {}