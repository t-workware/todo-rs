use std::collections::HashMap;
use std::rc::Rc;

use todo::error::TodoError;
use todo::lang::VecX;

#[derive(Clone, Debug, Default)]
pub struct Attrs {
    attrs: HashMap<String, String>,
    aliases: HashMap<String, Rc<String>>,
    keep_order_keys: Vec<Rc<String>>,
    pub keys: Vec<Rc<String>>,
    pub default_key: Rc<String>,
}

impl Attrs {
    #[inline]
    pub fn is_keep_order_key(&self, key: &Rc<String>) -> bool {
        self.keep_order_keys.contains(key)
    }

    pub fn add_keep_order_key(&mut self, key: &str) -> Rc<String> {
        let key = self.find_key(key).unwrap_or_else(|| {
            let key = Rc::new(key.to_string());
            self.keys.push(key.clone());
            key
        });
        if !self.is_keep_order_key(&key) {
            self.keep_order_keys.push(key.clone())
        }
        key
    }

    pub fn add_key(&mut self, key: &str) -> Rc<String> {
        self.find_key(key).unwrap_or_else(|| {
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

    pub fn add_aliases<A>(&mut self, key: &str, aliases: &[A]) -> Result<(), TodoError>
    where
        A: AsRef<str>,
    {
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
        let mut exist_key = false;
        let key = self.key_by_alias(alias)
            .and_then(|key| {
                if self.is_keep_order_key(&key) {
                    exist_key = true;
                    Some(key)
                } else {
                    self.keys.remove_element(&key)
                }
            })
            .unwrap_or_else(|| Rc::new(alias.to_string()));
        let string_key = (*key).clone();
        if !exist_key {
            self.keys.push(key);
        }
        self.attrs.insert(string_key, value.into())
    }

    pub fn set_default_attr<V: Into<String>>(&mut self, value: V) -> Option<String> {
        if !self.is_keep_order_key(&self.default_key) {
            if let Some(key) = self.keys.remove_element(&self.default_key) {
                self.keys.push(key);
            }
        }
        self.attrs.insert((*self.default_key).clone(), value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_order_keys() {
        let mut attrs = Attrs::default();

        attrs.set_attr_value("key A", "value A");
        attrs.set_attr_value("key B", "value B");
        attrs.set_attr_value("key C", "value C");
        attrs.set_attr_value("key B", "new value B");

        let results = [
            ("key A", "value A"),
            ("key C", "value C"),
            ("key B", "new value B"),
        ];
        for (i, key) in attrs.keys.iter().enumerate() {
            let key = key.as_str();
            let value = attrs.attr_value_as_str(key);
            assert_eq!(results[i], (key, value));
        }
    }

    #[test]
    fn keep_order_keys() {
        let mut attrs = Attrs::default();

        attrs.add_keep_order_key("key B");

        attrs.set_attr_value("key A", "value A");
        attrs.set_attr_value("key B", "value B");
        attrs.set_attr_value("key C", "value C");
        attrs.set_attr_value("key B", "new value B");

        let results = [
            ("key B", "new value B"),
            ("key A", "value A"),
            ("key C", "value C"),
        ];
        for (i, key) in attrs.keys.iter().enumerate() {
            let key = key.as_str();
            let value = attrs.attr_value_as_str(key);
            assert_eq!(results[i], (key, value));
        }
    }
}