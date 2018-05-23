use std::collections::HashMap;
use std::rc::Rc;
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
            .cloned()
    }

    pub fn key_by_alias(&self, alias: &str) -> Option<Rc<String>> {
        if let Some(key) = self.find_key(alias) {
            Some(key)
        } else {
            self.aliases.get(alias).cloned()
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
}
