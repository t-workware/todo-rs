use std::fs;
use todo::error::TodoError;
use todo::command::{Command, New};
use todo::command::store::Create as CanCreate;
use todo::tools::map_str;

#[derive(Clone, Debug, Default)]
pub struct Create {
    pub format: Option<String>,
    pub dir: Option<String>,
    pub ext: Option<String>,
    pub path: Option<String>
}

impl Command for Create {
    fn set_param(&mut self, key: &str, value: String) -> Result<(), TodoError> {
        match key.to_lowercase().as_str() {
            "ext" | "e" => self.ext = Some(value),
            "path" | "p" => self.path = Some(value),
            _ => return Err(TodoError::UnknownCommandParam { param: key.to_string() }),
        }
        Ok(())
    }

    fn exec(&mut self) {
        if let Some(ref dir) = self.dir {
            fs::create_dir_all(dir).expect(&format!("Can't create dir: {}", dir));
        }
        if let Some(ref path) = self.path {
            fs::File::create(path).expect(&format!("Creation error with path: {}", path));
            println!("create {}", path);
        }
    }
}

impl CanCreate for Create {
    fn init_from(&mut self, new: &New<Self>) {
        let mut format = map_str(&self.format, String::as_str).to_string();

        format.key_replace("id", map_str(&new.id,|id| id.0.as_str()));
        format.key_replace("top", map_str(&new.top,|top| top.0.as_str()));
        format.key_replace("scope", map_str(&new.scope,|scope| scope.0.as_str()));
        format.key_replace("name", map_str(&new.name,|name| name.as_str()));
        format.key_replace("ext", map_str(&self.ext,|ext| ext.as_str()));

        if let Some(ref dir) = self.dir {
            self.path = Some(format!("{}/{}", dir, format));
        }
    }
}

trait Format {
    fn find_from_pos(&self, pos: usize, needle: &str) -> Option<usize>;
    fn find_byte(&self, start: usize, needle: u8) -> Option<usize>;
    fn rfind_byte(&self, end: usize, needle: u8) -> Option<usize>;
    fn key_replace(&mut self, key: &str, value: &str);
}

impl Format for String {
    fn find_from_pos(&self, pos: usize, needle: &str) -> Option<usize> {
        let end = self.len() - needle.len() + 1;
        if pos < end {
            for i in pos..end {
                for j in 0..needle.len() {
                    if self.as_bytes()[i + j] != needle.as_bytes()[j] {
                        break;
                    }
                    if j == needle.len() - 1 {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    fn find_byte(&self, start: usize, needle: u8) -> Option<usize> {
        let source = self.as_bytes();
        for i in start..source.len() {
            if source[i] == needle {
                return Some(i);
            }
        }
        None
    }

    fn rfind_byte(&self, end: usize, needle: u8) -> Option<usize> {
        let source = self.as_bytes();
        for i in (0..end).rev() {
            if source[i] == needle {
                return Some(i);
            }
        }
        None
    }

    fn key_replace(&mut self, key: &str, value: &str) {
        let mut find_pos = 0;

        while let Some(index) = self.find_from_pos(find_pos, key) {
            find_pos = index + 1;

            if index > 0 && index + key.len() < self.as_bytes().len() {
                let (mut before, mut after) = ("".to_string(), "".to_string());
                let (start, end);

                if self.as_bytes()[index - 1] == b'{' {
                    start = index - 1;
                } else if index > 1 && self.as_bytes()[index - 1] == b':' {
                    if let Some(start_index) = self.rfind_byte(index - 1, b'{') {
                        start = start_index;
                        before = String::from_utf8_lossy(&self.as_bytes()[(start_index + 1)..(index - 1)]).to_string();
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }

                let index = index + key.len() - 1;
                if self.as_bytes()[index + 1] == b'}' {
                    end = index + 1;
                } else if index + 2 < self.as_bytes().len() && self.as_bytes()[index + 1] == b':' {
                    if let Some(end_index) = self.find_byte(index + 2, b'}') {
                        end = end_index;
                        after = String::from_utf8_lossy(&self.as_bytes()[(index + 2)..end_index]).to_string();
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }

                let head = String::from_utf8_lossy(&self.as_bytes()[..start]).to_string();
                let tail = String::from_utf8_lossy(&self.as_bytes()[(end + 1)..]).to_string();
                let body = if !value.is_empty() {
                    format!("{}{}{}", before, value, after)
                } else {
                    "".to_string()
                };

                find_pos = head.len() + body.len() + tail.len();
                *self = format!("{}{}{}", head, body, tail);
            }
        }
    }
}