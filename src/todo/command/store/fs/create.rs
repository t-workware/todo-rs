use std::fs;
use std::path::Path;
use std::io::Write;
use todo::attrs::Attrs;
use todo::error::TodoError;
use todo::command::Command;
use todo::command::store::Create as CanCreate;
use todo::command::store::fs::{Format, SequenceGenerator, AttrParser};
use todo::issue::{Content, Issue};

#[derive(Clone, Debug, Default)]
pub struct Create {
    content: String,
    pub attrs: Attrs,
    pub path: Option<String>,
    pub id_generator: Option<SequenceGenerator>,
}

#[derive(PartialEq)]
pub enum CreateAttr {
    IssuesDir,
    Format,
    Ext,
}

impl CreateAttr {
    pub fn by_key(key: &str) -> Option<Self> {
        Some(match key {
            key if CreateAttr::IssuesDir.key() == key => CreateAttr::IssuesDir,
            key if CreateAttr::Format.key() == key => CreateAttr::Format,
            key if CreateAttr::Ext.key() == key => CreateAttr::Ext,
            _ => return None,
        })
    }

    pub fn key(&self) -> &'static str {
        match *self {
            CreateAttr::IssuesDir => "issues_dir",
            CreateAttr::Format => "format",
            CreateAttr::Ext => "ext",
        }
    }
}

impl Command for Create {
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if let Some(key) = self.attrs.key_by_alias(param.to_lowercase().as_str()) {
            let attr = CreateAttr::by_key(key.as_str())
                .expect(&format!("{} command has `{}` key, but not support this attr", stringify!(Create), key));

            self.attrs.set_attr_value(attr.key(), value);
            Ok(())
        } else {
            Err(TodoError::UnknownCommandParam { param: param.to_string() })
        }
    }

    fn default_param_key(&self) -> &str {
        self.attrs.default_key.as_str()
    }

    fn exec(&mut self) {
        if let Some(ref str_path) = self.path {
            let path = Path::new(str_path);

            fs::File::open(path).expect_err(&format!("File {} already exists", str_path));
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir).expect(&format!("Can't create dir: {:?}", dir));
            }
            let mut file = fs::File::create(path)
                .expect(&format!("Creation error with path: {}", str_path));
            if !self.content.is_empty() {
                file.write_all(self.content.as_bytes())
                    .expect(&format!("Error content write to file: {}", str_path));
            }

            println!("{}", str_path);
        }
    }
}

impl CanCreate for Create {
    fn init_from<T: Content>(&mut self, issue: &Issue<T>) {
        let mut format = self.attrs.attr_value_as_str(CreateAttr::Format.key()).to_string();

        let mut id = issue.get_id().map(|id| id.clone()).unwrap_or_default();

        if let Some(ref generator) = self.id_generator {
            let id_found = format.find(&issue.id_attr_key)
                .and_then(|pos| format.key_replaceable_pos(pos, issue.id_attr_key.len()))
                .is_some();
            if id_found && issue.get_id().is_none() {
                id = generator.next().expect("Generate next id fail");
            }
        }

        format.key_replace(&issue.id_attr_key, id.as_str());
        format.key_replace(CreateAttr::Ext.key(), self.attrs.attr_value_as_str(CreateAttr::Ext.key()));
        for key in issue.attrs.keys.iter() {
            let key = key.as_str();
            if key != issue.id_attr_key {
                let value = issue.attrs.attr_value_as_str(key);
                if !format.key_replace(key, value) {
                    self.content += &format!("{}\n", AttrParser::encode_attr(key, value));
                }
            }
        }

        if let Some(dir) = self.attrs.attr_value(CreateAttr::IssuesDir.key()) {
            self.path = Some(format!("{}/{}", dir, format));
        }
    }
}