use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use regex::Regex;
use failure::Error;
use walkdir::{DirEntry, WalkDir};

use todo::attrs::Attrs;
use todo::command::store::Find as CanFind;
use todo::command::store::fs::AttrParser;
use todo::command::{Command, IssueCommand};
use todo::error::TodoError;
use todo::issue::{Content, Issue};

#[derive(Clone, Debug)]
pub struct Find {
    issue_attrs: Option<Attrs>,
    pub attrs: Attrs,
    pub filter: Option<Regex>,
}

#[derive(EnumIterator, PartialEq)]
pub enum FindAttr {
    IssuesDir,
    Capture,
    Filter,
    All,
}

impl FindAttr {
    pub fn by_key(key: &str) -> Option<Self> {
        Some(match key {
            key if FindAttr::IssuesDir.key() == key => FindAttr::IssuesDir,
            key if FindAttr::Capture.key() == key => FindAttr::Capture,
            key if FindAttr::Filter.key() == key => FindAttr::Filter,
            key if FindAttr::All.key() == key => FindAttr::All,
            _ => return None,
        })
    }

    pub fn key(&self) -> &'static str {
        match *self {
            FindAttr::IssuesDir => "issues_dir",
            FindAttr::Capture => "capture",
            FindAttr::Filter => "filter",
            FindAttr::All => "all",
        }
    }
}

impl Find {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.len() > 1 && s.starts_with('.'))
            .unwrap_or(false)
    }

    pub fn all(&self) -> bool {
        self.attrs
            .attr_value(FindAttr::All.key())
            .map(|value| {
                !["false", "f", "not", "no", "n", "0"].contains(&value.to_lowercase().as_str())
            })
            .unwrap_or(false)
    }

    pub fn capture_regex(&self) -> Option<Regex> {
        self.attrs
            .attr_value(FindAttr::Capture.key())
            .and_then(|value| {
                if !value.is_empty() {
                    Some(
                        Regex::new(value.as_str())
                            .expect(&format!("It isn't correct regex string: `{}`", value))
                    )
                } else {
                    None
                }
            })
    }

    pub fn walk_through_issues(&self, root: &Path) -> Result<(), Error> {
        let walker = WalkDir::new(root)
            .follow_links(true)
            .into_iter();
        let issues_dir = OsStr::new(self.attrs.attr_value_as_str(FindAttr::IssuesDir.key()));
        let capture_regex = self.capture_regex();

        for entry in walker.filter_entry(
            |e| self.all() || !Find::is_hidden(e)
        ) {
            let entry = entry?;
            let mut prefix_path = PathBuf::new();
            if entry.file_type().is_file() {
                'path_chunks: for chunk in entry.path().iter() {
                    prefix_path = prefix_path.join(chunk);

                    if issues_dir.is_empty() || chunk == issues_dir {
                        let maybe_path = entry.path()
                            .as_os_str()
                            .to_str()
                            .map(|path| &path[2..]); // trim_left_matches("./")

                        if let Some(path) = maybe_path {
                            if self.filter.is_none()
                                || self.filter.as_ref().unwrap().is_match(path)
                            {
                                if let Some(ref issue_attrs) = self.issue_attrs {
                                    if issue_attrs.count() > 0 {
                                        let mut attrs = Vec::new();

                                        if let Some(ref regex) = capture_regex {
                                            let prefix_len = prefix_path.to_str()
                                                .map(|path| {
                                                    let len = path[2..].len();
                                                    if len > 0 { len + 1 } else { len }
                                                })
                                                .unwrap_or(0);
                                            if let Some(caps) = regex.captures(&path[prefix_len..]) {
                                                for maybe_name in regex.capture_names() {
                                                    let name = maybe_name.unwrap_or("");
                                                    if !name.is_empty() {
                                                        let text = caps.name(name)
                                                            .map(|m| m.as_str())
                                                            .unwrap_or("");
                                                        attrs.push((name.to_string(), text.to_string()));
                                                    }
                                                }
                                            }
                                        }

                                        let file = File::open(path)?;
                                        let parser = AttrParser::new();
                                        attrs.extend(parser.read_attrs(file)?);

                                        for (key, value) in issue_attrs.iter() {
                                            if attrs.iter().find(|&&(ref attr_key, ref attr_value)| {
                                                attr_key == key && parser.parse_value(attr_value.as_str()).0 == *value
                                            }).is_none() {
                                                break 'path_chunks;
                                            }
                                        }
                                    }
                                }
                                println!("{}", path);
                            }
                            break 'path_chunks;
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
            issue_attrs: None,
            attrs,
            filter: Default::default(),
        }
    }
}

impl Command for Find {
    fn set_param(&mut self, param: &str, value: String) -> Result<(), TodoError> {
        if let Some(key) = self.attrs.key_by_alias(param.to_lowercase().as_str()) {
            let attr = FindAttr::by_key(key.as_str())
                .expect(&format!(
                    "{} command has `{}` key, but not support this attr",
                    stringify!(Find),
                    key
                ));

            if let FindAttr::Filter = attr {
                self.filter = Some(
                    Regex::new(&value)
                        .expect(&format!("Invalid filter regular expression: {}", value)),
                )
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

impl IssueCommand for Find {
    fn init_from<T: Content>(&mut self, issue: &Issue<T>) {
        self.issue_attrs = Some(issue.attrs.clone());
    }
}

impl CanFind for Find {}