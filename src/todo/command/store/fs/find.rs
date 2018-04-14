use std::path::Path;
use std::ffi::OsStr;
use walkdir::WalkDir;
use failure::Error;
use todo::command::Command;
use todo::command::store::Find as CanFind;
use todo::error::TodoError;

#[derive(Clone, Debug, Default)]
pub struct Find {
    pub format: Option<String>,
    pub dir: String,
    pub exts: Option<Vec<String>>,
}

impl Find {
    pub fn walk_through_issues(&self, root: &Path) -> Result<(), Error> {
        let walker = WalkDir::new(root)
            .follow_links(true)
            .into_iter();
        let issues_dir = OsStr::new(&self.dir);

        for entry in walker {
            let entry = entry?;
            if entry.file_type().is_file() {
                for chunk in entry.path().iter() {
                    if issues_dir.is_empty() || chunk == issues_dir {
                        let path = match entry.path().strip_prefix("./") {
                            Ok(path) => path,
                            _ => entry.path()
                        };
                        println!("{}", path.display());
                        break;
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
