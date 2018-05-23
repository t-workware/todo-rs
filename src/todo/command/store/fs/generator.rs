use failure::Error;
use std::fs;
use std::io::{Read, Write};
use todo::error::TodoError;

#[derive(Clone, Debug, Default)]
pub struct SequenceGenerator {
    pub required: bool,
    pub file: Option<String>,
}

impl SequenceGenerator {
    pub fn next(&self) -> Result<String, Error> {
        match self.file {
            Some(ref path) => {
                let id = {
                    let mut contents = String::new();
                    let open = fs::File::open(path);
                    match open {
                        Ok(mut file) => {
                            file.read_to_string(&mut contents)?;
                            contents.trim().to_string()
                        }
                        Err(err) => {
                            return if self.required {
                                Err(err.into())
                            } else {
                                Ok("".to_string())
                            }
                        }
                    }
                };
                let new_id = format!("{}", id.parse::<u64>()? + 1);

                let mut file = fs::File::create(path)?;
                file.write_all(new_id.as_bytes())?;

                Ok(id)
            }
            None if self.required => Err(TodoError::FileNotSpecified.into()),
            _ => Ok("".to_string()),
        }
    }
}
