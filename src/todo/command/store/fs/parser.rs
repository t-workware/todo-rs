use regex::Regex;
use todo::attrs::Attrs;

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

    pub fn parse_and_set_attr<L>(&self, line: L, attrs: &mut Attrs) -> Option<String>
        where
            L: AsRef<str>
    {
        if let Some((key, value)) = self.parse_attr(line) {
            attrs.set_attr_value(key.as_str(), value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use todo::attrs::Attrs;
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

        assert_eq!(None, parser.parse_and_set_attr("#[key]", &mut attrs));
        assert_eq!(None, attrs.attr_value("key"));

        assert_eq!(0, attrs.keys.len());

        assert_eq!(None, parser.parse_and_set_attr("#[key: value]", &mut attrs));
        assert_eq!(Some("value"), attrs.attr_value("key")
            .map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(Some("value"), parser.parse_and_set_attr("#[ key :  value 2 ]", &mut attrs)
            .as_ref().map(String::as_str));
        assert_eq!(Some("value 2"), attrs.attr_value("key")
            .map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(None, parser.parse_and_set_attr("#[key 1: [1, 2]]", &mut attrs));
        assert_eq!(Some("[1, 2]"), attrs.attr_value("key 1")
            .map(String::as_str));

        assert_eq!(2, attrs.keys.len());
    }
}