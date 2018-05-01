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

impl<T> Command for New<T>
    where T: Create
{
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if !param.is_empty() {
            let mut is_create_param = false;
            if let Some(create) = self.create.as_mut() {
                is_create_param = create.set_param(param, value.clone()).is_ok();
            }
            if !is_create_param {
                self.issue.attrs.set_attr_value(param.to_lowercase().as_str(), value);
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