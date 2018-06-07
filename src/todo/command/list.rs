use std::mem;
use std::result::Result;

use todo::command::store::Find;
use todo::command::Command;
use todo::error::TodoError;
use todo::issue::Issue;

#[derive(Clone, Debug, Default)]
pub struct List<T>
where
    T: Find,
{
    pub find: Option<T>,
    pub issue: Issue<String>,
}

impl<T> Command for List<T>
where
    T: Find,
{
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if !param.is_empty() {
            let mut is_find_param = false;
            if let Some(find) = self.find.as_mut() {
                is_find_param = find.set_param(param, value.clone()).is_ok();
            }
            if !is_find_param {
                self.issue.attrs.set_attr_value(param.to_lowercase().as_str(), value);
            }
        } else if let Some(find) = self.find.as_mut() {
            let default_key = find.default_param_key().to_string();
            find.set_param(&default_key, value)?;
        }
        Ok(())
    }

    fn default_param_key(&self) -> &str {
        self.find
            .as_ref()
            .map(|find| find.default_param_key())
            .expect("Find command not exist")
    }

    fn exec(&mut self) {
        let mut find =  mem::replace(&mut self.find, None)
            .expect("Find command not exist");

        find.init_from(&self.issue);
        find.exec();
        self.find = Some(find);
    }
}
