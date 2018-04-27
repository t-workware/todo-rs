use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};
use failure::Error;
use todo::command::Command;
use todo::command::store::Find as CanFind;
use todo::error::TodoError;

#[derive(Clone, Debug, Default)]
pub struct Find {
    pub format: Option<String>,
    pub filter: Option<Regex>,
    pub all: bool,
    pub dir: String,
    pub exts: Option<Vec<String>>,
}

impl Find {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.len() > 1 && s.starts_with("."))
            .unwrap_or(false)
    }

    pub fn walk_through_issues(&self, root: &Path) -> Result<(), Error> {
        let walker = WalkDir::new(root)
            .follow_links(true)
            .into_iter();
        let issues_dir = OsStr::new(&self.dir);

        for entry in walker.filter_entry(|e|
            self.all || !Find::is_hidden(e)
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

impl CanFind for Find {}

impl Command for Find {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        match key.to_lowercase().as_str() {
            "filter" | "f" => self.filter = Some(Regex::new(&value)
                .expect(&format!("Invalid filter regular expression: {}", value))),
            "all" | "a" => self.all = if ["false", "f", "not", "no", "n", "0"]
                .contains(&value.to_lowercase().as_str()) {false} else {true},
            "dir" | "d" => self.dir = value,
            _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() }),
        }
        Ok(())
    }

    fn exec(&mut self) {
        let root = Path::new(".");
        self.walk_through_issues(&root)
            .expect(&format!("Can't walk through subdir of `{}`", root.display()));
    }
}
