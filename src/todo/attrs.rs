use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;
use todo::error::TodoError;

#[derive(Clone, Debug, Default)]
pub struct Attrs {
    attrs: HashMap<String, String>,
    aliases: HashMap<String, Rc<String>>,
    pub keys: Vec<Rc<String>>,
    pub default_key: Rc<String>,
}

impl Attrs {
    pub fn add_key(&mut self, key: &str) -> Rc<String> {
        self.find_key(key)
            .unwrap_or_else(|| {
                let key = Rc::new(key.to_string());
                self.keys.push(key.clone());
                key
            })
    }

    pub fn find_key(&self, key: &str) -> Option<Rc<String>> {
        self.keys.iter()
            .find(|&item| item.as_str() == key)
            .map(|item| item.clone())
    }

    pub fn key_by_alias(&self, alias: &str) -> Option<Rc<String>> {
        if let Some(key) = self.find_key(alias) {
            Some(key)
        } else {
            self.aliases.get(alias).map(|key| key.clone())
        }
    }

    pub fn add_aliases<A: AsRef<str>>(&mut self, key: &str, aliases: &[A]) -> Result<(), TodoError> {
        if let Some(key) = self.find_key(key) {
            for alias in aliases {
                let alias = alias.as_ref().to_string();
                if self.aliases.contains_key(&alias) {
                    return Err(TodoError::AliasAlreadyExists { alias, key: (*key).clone() });
                }
                self.aliases.insert(alias, key.clone());
            }
            Ok(())
        } else {
            Err(TodoError::KeyNotFound { key: key.to_string() })
        }
    }

    pub fn attr_value(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }

    pub fn attr_value_as_str(&self, key: &str) -> &str {
        self.attrs.get(key).map(|s| s.as_str()).unwrap_or_default()
    }

    pub fn set_attr_value<V: Into<String>>(&mut self, alias: &str, value: V) -> Option<String> {
        let key = self.key_by_alias(alias).unwrap_or_else(|| {
            let key = Rc::new(alias.to_string());
            self.keys.push(key.clone());
            key
        });
        self.attrs.insert((*key).clone(), value.into())
    }

    pub fn set_default_attr<V: Into<String>>(&mut self, value: V) -> Option<String> {
        self.attrs.insert((*self.default_key).clone(), value.into())
    }

    pub fn parse_and_set_attr<L>(&mut self, line: L, parser: &AttrParser) -> Option<String>
    where
        L: AsRef<str>
    {
        if let Some((key, value)) = parser.parse_attr(line) {
            self.set_attr_value(key.as_str(), value)
        } else {
            None
        }
    }
}

pub struct AttrParser {
    pub regex: Regex,
}

impl AttrParser {
    pub fn new() -> Self {
        let regex = r"^\#\[(?s)(?P<key>.+):(?P<value>.*)\]$";
        AttrParser {
            regex: Regex::new(regex)
                .expect(&format!("`{}` is not regular expression", regex))
        }
    }

    pub fn encode_attr<K, V>(key: K, value: V) -> String
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        format!("#[{}: {}]", key.as_ref(), value.as_ref())
    }

    pub fn parse_attr<L>(&self, line: L) -> Option<(String, String)>
    where
        L: AsRef<str>,
    {
        for cap in self.regex.captures_iter(line.as_ref()) {
            return Some((cap[1].trim().to_string(), cap[2].trim().to_string()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_attr() {
        assert_eq!("#[key: value]", AttrParser::encode_attr("key", "value"));
        assert_eq!("#[key 1: value 1, value 2]", AttrParser::encode_attr("key 1", "value 1, value 2"));
    }

    #[test]
    fn parse_attr() {
        let parser = AttrParser::new();

        assert_eq!(None, parser.parse_attr("test"));
        assert_eq!(None, parser.parse_attr("#[]"));
        assert_eq!(None, parser.parse_attr("#[key]"));
        assert_eq!(Some(("key".to_string(), "value".to_string())),
                   parser.parse_attr("#[key:value]"));
        assert_eq!(Some(("key".to_string(), "value".to_string())),
                   parser.parse_attr("#[key: value]"));
        assert_eq!(Some(("key".to_string(), "value".to_string())),
                   parser.parse_attr("#[ key : value ]"));
        assert_eq!(Some(("key".to_string(), "value".to_string())),
                   parser.parse_attr("#[\tkey : \nvalue\n]"));
        assert_eq!(Some(("key 1".to_string(), "value 1".to_string())),
                   parser.parse_attr("#[key 1: value 1]"));
        assert_eq!(Some(("key 1".to_string(), "[value 1, value 2]".to_string())),
                   parser.parse_attr("#[key 1: [value 1, value 2]]"));
    }

    #[test]
    fn parse_and_set_attr() {
        let mut attrs = Attrs::default();
        let parser = AttrParser::new();

        assert_eq!(0, attrs.keys.len());

        assert_eq!(None, attrs.parse_and_set_attr("#[key]", &parser));
        assert_eq!(None, attrs.attr_value("key"));

        assert_eq!(0, attrs.keys.len());

        assert_eq!(None, attrs.parse_and_set_attr("#[key: value]", &parser));
        assert_eq!(Some("value"), attrs.attr_value("key")
            .map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(Some("value"), attrs.parse_and_set_attr("#[ key :  value 2 ]", &parser)
            .as_ref().map(String::as_str));
        assert_eq!(Some("value 2"), attrs.attr_value("key")
            .map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(None, attrs.parse_and_set_attr("#[key 1: [1, 2]]", &parser));
        assert_eq!(Some("[1, 2]"), attrs.attr_value("key 1")
            .map(String::as_str));

        assert_eq!(2, attrs.keys.len());
    }
}