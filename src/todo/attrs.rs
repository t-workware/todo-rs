use std::collections::{HashMap, hash_map::Iter};
use todo::error::TodoError;

#[derive(Clone, Debug, Default)]
pub struct Attrs {
    names: Vec<(String, Vec<String>)>,
    attrs: HashMap<String, String>,
    pub default_key: String,
}

impl Attrs {
    pub fn names(&self) -> &[(String, Vec<String>)] {
        &self.names
    }

    pub fn names_mut(&mut self) -> &mut Vec<(String, Vec<String>)> {
        &mut self.names
    }

    pub fn set_names(&mut self, names: Vec<(String, Vec<String>)>) {
        self.names = names;
    }

    pub fn get_aliases(&self, attr_key: &str) -> Option<&Vec<String>> {
        for &(ref key, ref aliases) in &self.names {
            if attr_key == *key {
                return Some(aliases);
            }
        }
        None
    }

    pub fn get_key(&self, alias: &str) -> Option<&str> {
        for &(ref key, ref aliases) in &self.names {
            if alias == *key {
                return Some(key.as_str());
            }
            for name in aliases {
                if alias == *name {
                    return Some(key.as_str());
                }
            }
        }
        None
    }

    pub fn add_name<K: Into<String>>(&mut self, attr_key: K, aliases: Vec<String>) {
        self.names.push((attr_key.into(), aliases));
    }

    pub fn add_name_alias<K: Into<String>, A: Into<String>>(&mut self, attr_key: K, alias: A) {
        let attr_key = attr_key.into();
        let alias = alias.into();
        for &mut (ref key, ref mut aliases) in &mut self.names {
            if attr_key == *key {
                aliases.push(alias);
                return;
            }
        }
        self.names.push((attr_key.into(), vec![alias]));
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.attrs.iter()
    }

    pub fn get_attr(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }

    pub fn set_attr<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) -> Result<Option<String>, TodoError> {
        let key = key.into();
        if self.get_aliases(&key).is_some() {
            Ok(self.attrs.insert(key, value.into()))
        } else {
            Err(TodoError::UnknownAttribute { attr: key })
        }
    }

    pub fn set_attr_by_alias<V: Into<String>>(&mut self, alias: &str, value: V) -> Option<Option<String>> {
        self.get_key(alias)
            .map(|key| key.to_string())
            .and_then(|key| {
                Some(self.attrs.insert(key, value.into()))
            })
    }

    pub fn set_default_attr<V: Into<String>>(&mut self, value: V) -> Option<String> {
        self.attrs.insert(self.default_key.clone(), value.into())
    }
}