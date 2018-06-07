use std::io::{BufRead, BufReader, Read};
use std::str;
use failure::Error;
use regex::Regex;

use todo::attrs::Attrs;

pub struct AttrParser {
    pub attr_regex: Regex,
    pub expr_regex: Regex,
}

impl AttrParser {
    pub fn new() -> Self {
        let attr_regex = r"^\#\[(?s)(?P<key>.+):(?P<value>.*)\]$";
        let expr_regex = r"^(?s)(?P<actual_value>.*)\s=\s(?P<expr>if\s.+)$";
        AttrParser {
            attr_regex: Regex::new(attr_regex)
                .expect(&format!("`{}` is not regular expression", attr_regex)),
            expr_regex: Regex::new(expr_regex)
                .expect(&format!("`{}` is not regular expression", expr_regex)),
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
        if let Some(cap) = self.attr_regex.captures_iter(line.as_ref()).next() {
            Some((cap[1].trim().to_string(), cap[2].trim().to_string()))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn parse_and_set_attr<L>(&self, line: L, attrs: &mut Attrs) -> Option<String>
    where
        L: AsRef<str>,
    {
        if let Some((key, value)) = self.parse_attr(line) {
            attrs.set_attr_value(key.as_str(), value)
        } else {
            None
        }
    }

    pub fn parse_value<V>(&self, value: V) -> (String, Option<String>)
    where
        V: Into<String> + AsRef<str>,
    {
        if let Some(cap) = self.expr_regex.captures_iter(value.as_ref()).next() {
            return (cap[1].trim().to_string(), Some(cap[2].trim().to_string()));
        }
        (value.into(), None)
    }

    pub fn read_attrs<R>(&self, source: R) -> Result<Vec<(String, String)>, Error>
    where
        R: Read,
    {
        let mut attrs = Vec::new();

        let mut reader = BufReader::new(source);
        let mut buf = Vec::<u8>::new();
        let mut attr = String::new();
        let mut open_brackets = 0;

        while reader.read_until(b'\n', &mut buf)? != 0 {
            if buf.starts_with(&[b'#', b'[']) || !attr.is_empty() {
                let mut in_progress = true;
                for (i, &bch) in buf.iter().enumerate() {
                    match bch {
                        b'[' => {
                            open_brackets += 1;
                        }
                        b']' => {
                            open_brackets -= 1;

                            if open_brackets == 0 {
                                attr += str::from_utf8(&buf[..(i + 1)])?;
                                let _ = self.parse_attr(&attr)
                                    .map(|parsed_attr| attrs.push(parsed_attr));
                                in_progress = false;
                                attr.clear();
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if in_progress {
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
    use super::*;
    use todo::attrs::Attrs;
    use todo::lang::ToStrings;

    #[test]
    fn encode_attr() {
        assert_eq!("#[key: value]", AttrParser::encode_attr("key", "value"));
        assert_eq!(
            "#[key 1: value 1, value 2]",
            AttrParser::encode_attr("key 1", "value 1, value 2")
        );
    }

    #[test]
    fn parse_attr() {
        let parser = AttrParser::new();

        assert_eq!(None, parser.parse_attr("test"));
        assert_eq!(None, parser.parse_attr("#[]"));
        assert_eq!(None, parser.parse_attr("#[key]"));
        assert_eq!(
            Some(("key", "").to_strings()),
            parser.parse_attr("#[key:]")
        );
        assert_eq!(
            Some(("key", "value").to_strings()),
            parser.parse_attr("#[key:value]")
        );
        assert_eq!(
            Some(("key", "value").to_strings()),
            parser.parse_attr("#[key: value]")
        );
        assert_eq!(
            Some(("key", "value").to_strings()),
            parser.parse_attr("#[ key : value ]")
        );
        assert_eq!(
            Some(("key", "value").to_strings()),
            parser.parse_attr("#[\tkey : \nvalue\n]")
        );
        assert_eq!(
            Some(("key 1", "value 1").to_strings()),
            parser.parse_attr("#[key 1: value 1]")
        );
        assert_eq!(
            Some(("key 1", "[value 1, value 2]").to_strings()),
            parser.parse_attr("#[key 1: [value 1, value 2]]")
        );
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
        assert_eq!(Some("value"), attrs.attr_value("key").map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(
            Some("value"),
            parser
                .parse_and_set_attr("#[ key :  value 2 ]", &mut attrs)
                .as_ref()
                .map(String::as_str)
        );
        assert_eq!(Some("value 2"), attrs.attr_value("key").map(String::as_str));

        assert_eq!(1, attrs.keys.len());

        assert_eq!(
            None,
            parser.parse_and_set_attr("#[key 1: [1, 2]]", &mut attrs)
        );
        assert_eq!(
            Some("[1, 2]"),
            attrs.attr_value("key 1").map(String::as_str)
        );

        assert_eq!(2, attrs.keys.len());
    }

    #[test]
    fn parse_value() {
        let parser = AttrParser::new();

        assert_eq!(("", None).to_strings(), parser.parse_value(""));
        assert_eq!((" = ", None).to_strings(), parser.parse_value(" = "));
        assert_eq!(("test", None).to_strings(), parser.parse_value("test"));
        assert_eq!(("test =", None).to_strings(), parser.parse_value("test ="));
        assert_eq!(
            ("test = some", None).to_strings(),
            parser.parse_value("test = some")
        );
        assert_eq!(
            ("test = if", None).to_strings(),
            parser.parse_value("test = if")
        );
        assert_eq!(
            ("test", Some("if cond")).to_strings(),
            parser.parse_value("test = if cond")
        );
        assert_eq!(
            ("true", Some("if some\nthen \"true\" else \"false\"")).to_strings(),
            parser.parse_value("true =\nif some\nthen \"true\" else \"false\"\n")
        );
        assert_eq!(
            ("", Some("if some\nthen \"true\" else \"\"")).to_strings(),
            parser.parse_value(" =\nif some\nthen \"true\" else \"\"\n")
        );
    }

    #[test]
    fn read_attrs() {
        let parser = AttrParser::new();

        let source = "";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert!(attrs.is_empty());

        let source = "[key: value]";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert!(attrs.is_empty());

        let source = "#[key: value";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert!(attrs.is_empty());

        let source = "#[key: value]";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!([("key", "value")].to_strings(), attrs);

        let source = "\n#[\nkey:\n value\n]\n";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!([("key", "value")].to_strings(), attrs);

        let source = "#[key: value] // attr";
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!([("key", "value")].to_strings(), attrs);

        let source = r#"
#[key: value]
// #[test: some]
test
#[key 2: value 2] // new value
 #[test 2: some 2]
        "#;
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!([("key", "value"), ("key 2", "value 2")].to_strings(), attrs);

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
        let attrs = parser
            .read_attrs(source.as_bytes())
            .expect("Read attrs error");
        assert_eq!(
            [
                ("key", "value []"),
                ("key", "value [new]"),
                ("test", "[some\n[2]]")
            ].to_strings(),
            attrs
        );
    }
}
