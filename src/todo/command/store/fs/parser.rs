use std::str;
use std::io::{Read, BufRead, BufReader};
use regex::Regex;
use failure::Error;
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

    pub fn read_attrs<R>(&self, inner: R) -> Result<Vec<(String, String)>, Error>
        where
            R: Read,
    {
        let mut attrs = Vec::new();

        let mut reader = BufReader::new(inner);
        let mut buf = Vec::<u8>::new();
        let mut attr = String::new();
        let mut open_brackets = 0;

        while reader.read_until(b'\n', &mut buf)? != 0 {
            if buf.starts_with(&[b'#', b'[']) || !attr.is_empty() {
                let mut parsed = false;
                for (i, &bch) in buf.iter().enumerate() {
                    match bch {
                        b'[' => {
                            open_brackets += 1;
                        },
                        b']' => {
                            open_brackets -= 1;

                            if open_brackets == 0 {
                                attr += str::from_utf8(&buf[..(i + 1)])?;
                                self.parse_attr(&attr)
                                    .map(|parsed_attr| attrs.push(parsed_attr))
                                    .ok_or(format_err!("Can't parse attr `{}`", attr))?;
                                attr.clear();
                                parsed = true;
                                break;
                            }
                        },
                        _ => {}
                    }
                }
                if !parsed {
                    attr += str::from_utf8(&buf)?;
                }
            }
            buf.clear();
        }
        Ok(attrs)
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
        assert_eq!(Some(("key".to_string(), "".to_string())),
                   parser.parse_attr("#[key:]"));
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

    #[test]
    fn read_attrs() {
        let as_attrs = |array: &[(&str, &str)]| {
            array.iter().map(|x| (x.0.to_string(), x.1.to_string())).collect::<Vec<(String, String)>>()
        };
        let parser = AttrParser::new();

        let source = "";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[]), attrs);

        let source = "[key: value]";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[]), attrs);

        let source = "#[key: value";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[]), attrs);

        let source = "#[key: value]";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[("key", "value")]), attrs);

        let source = "\n#[\nkey:\n value\n]\n";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[("key", "value")]), attrs);

        let source = "#[key: value] // attr";
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[("key", "value")]), attrs);

        let source = r#"
#[key: value]
// #[test: some]
test
#[key 2: value 2] // new value
 #[test 2: some 2]
        "#;
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[("key", "value"), ("key 2", "value 2")]), attrs);

        let source = r#"
#[key: value []]
// #[test: some]
test
#[key: value [new]] // new value
 #[test: [some 1]]
#[test:
[some
[2]]]
        "#;
        let attrs = parser.read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(as_attrs(&[
            ("key", "value []"),
            ("key", "value [new]"),
            ("test", "[some\n[2]]")
        ]), attrs);
    }
}