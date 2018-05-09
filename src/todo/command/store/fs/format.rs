pub trait Format {
    fn find_from_pos(&self, pos: usize, needle: &str) -> Option<usize>;
    fn find_byte(&self, start: usize, needle: u8) -> Option<usize>;
    fn rfind_byte(&self, end: usize, needle: u8) -> Option<usize>;
    fn key_replaceable_pos(&self, key_pos: usize, key_len: usize) -> Option<(usize, usize)>;
    fn key_replace(&mut self, key: &str, value: &str) -> bool;
}

impl Format for String {
    fn find_from_pos(&self, pos: usize, needle: &str) -> Option<usize> {
        if needle.len() > self.len() {
            return None;
        }

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

    fn key_replaceable_pos(&self, key_pos: usize, key_len: usize) -> Option<(usize, usize)> {
        let (mut start, mut end) = (0, 0);
        let mut found = false;

        let index = key_pos;
        if index > 0 && index + key_len < self.len() {
            if self.as_bytes()[index - 1] == b'{' {
                start = index - 1;
                found = true;
            } else if index > 1 && self.as_bytes()[index - 1] == b':' {
                if let Some(start_index) = self.rfind_byte(index - 1, b'{') {
                    start = start_index;
                    found = true;
                }
            }
        }

        if found {
            found = false;
            let index = index + key_len - 1;

            if self.as_bytes()[index + 1] == b'}' {
                end = index + 1;
                found = true;
            } else if index + 2 < self.as_bytes().len() && self.as_bytes()[index + 1] == b':' {
                if let Some(end_index) = self.find_byte(index + 2, b'}') {
                    end = end_index;
                    found = true;
                }
            }
        }

        if found {
            Some((start, end))
        } else {
            None
        }
    }

    fn key_replace(&mut self, key: &str, value: &str) -> bool {
        let mut replaced = false;
        let mut find_pos = 0;

        while let Some(index) = self.find_from_pos(find_pos, key) {
            find_pos = index + 1;

            if let Some((start, end)) = self.key_replaceable_pos(index, key.len()) {
                let before = if start + 1 < index - 1 {
                    String::from_utf8_lossy(&self.as_bytes()[(start + 1)..(index - 1)]).to_string()
                } else {
                    "".to_string()
                };

                let after = if index + key.len() + 1 < end {
                    String::from_utf8_lossy(&self.as_bytes()[(index + key.len() + 1)..end]).to_string()
                } else {
                    "".to_string()
                };

                let head = String::from_utf8_lossy(&self.as_bytes()[..start]).to_string();
                let tail = String::from_utf8_lossy(&self.as_bytes()[(end + 1)..]).to_string();

                let body = if !value.is_empty() {
                    format!("{}{}{}", before, value, after)
                } else {
                    "".to_string()
                };

                find_pos = head.len() + body.len() + tail.len();
                *self = format!("{}{}{}", head, body, tail);
                replaced = true;
            }
        }
        replaced
    }
}