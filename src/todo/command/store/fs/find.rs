use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};
use failure::Error;
use todo::attrs::Attrs;
use todo::command::Command;
use todo::command::store::Find as CanFind;
use todo::error::TodoError;

#[derive(Clone, Debug)]
pub struct Find {
    pub attrs: Attrs,
    pub filter: Option<Regex>,
}

#[derive(EnumIterator, PartialEq)]
pub enum FindAttr {
    Format,
    Filter,
    All,
    Dir
}

impl FindAttr {
    pub fn by_key(key: &str) -> Option<Self> {
        Some(match key {
            key if FindAttr::Format.key() == key => FindAttr::Format,
            key if FindAttr::Filter.key() == key => FindAttr::Filter,
            key if FindAttr::All.key() == key => FindAttr::All,
            key if FindAttr::Dir.key() == key => FindAttr::Dir,
            _ => return None,
        })
    }

    pub fn key(&self) -> &'static str {
        match *self {
            FindAttr::Format => "format",
            FindAttr::Filter => "filter",
            FindAttr::All => "all",
            FindAttr::Dir => "dir"
        }
    }
}

impl Find {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.len() > 1 && s.starts_with("."))
            .unwrap_or(false)
    }

    pub fn all(&self) -> bool {
        self.attrs.attr_value(FindAttr::All.key())
            .map(|value| {
                !["false", "f", "not", "no", "n", "0"].contains(&value.to_lowercase().as_str())
            })
            .unwrap_or(false)
    }

    pub fn walk_through_issues(&self, root: &Path) -> Result<(), Error> {
        let walker = WalkDir::new(root)
            .follow_links(true)
            .into_iter();
        let issues_dir = OsStr::new(self.attrs.attr_value_as_str(FindAttr::Dir.key()));

        for entry in walker.filter_entry(|e|
            self.all() || !Find::is_hidden(e)
        ) {
            let entry = entry?;
            if entry.file_type().is_file() {
                for chunk in entry.path().iter() {
                    if issues_dir.is_empty() || chunk == issues_dir {
                        if let Some(path) = match entry.path().strip_prefix("./") {
                            Ok(path) => path,
                            _ => entry.path()
                        }.as_os_str().to_str() {
                            if self.filter.is_none() || self.filter.as_ref().unwrap().is_match(path) {
                                println!("{}", path);
                            }
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for Find {
    fn default() -> Self {
        let mut attrs = Attrs::default();
        for variant in FindAttr::iter_variants() {
            let key = attrs.add_key(variant.key());
            if variant == FindAttr::Filter {
                attrs.default_key = key;
            }
        }

        Find {
            attrs,
            filter: Default::default()
        }
    }
}

impl CanFind for Find {}

impl Command for Find {
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if let Some(key) = self.attrs.key_by_alias(param.to_lowercase().as_str()) {
            let attr = FindAttr::by_key(key.as_str())
                .expect(&format!("{} command has `{}` key, but not support this attr", stringify!(Find), key));

            match attr {
                FindAttr::Filter => self.filter = Some(Regex::new(&value)
                    .expect(&format!("Invalid filter regular expression: {}", value))),
                _ => ()
            }
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
        let root = Path::new(".");
        self.walk_through_issues(&root)
            .expect(&format!("Can't walk through subdir of `{}`", root.display()));
    }
}
