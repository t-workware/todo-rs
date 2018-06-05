use std::iter::FromIterator;

pub trait ToStrings<T> {
    fn to_strings(&self) -> T;
}

pub trait ToStringsCollect {
    type Item;

    fn to_strings_collect<T>(&self) -> T
    where
        T: FromIterator<Self::Item>;
}

impl<'a, 'b> ToStrings<(String, String)> for (&'a str, &'b str) {
    fn to_strings(&self) -> (String, String) {
        (self.0.to_string(), self.1.to_string())
    }
}

impl<'a, 'b> ToStrings<Option<(String, String)>> for Option<(&'a str, &'b str)> {
    fn to_strings(&self) -> Option<(String, String)> {
        self.as_ref().map(ToStrings::<(String, String)>::to_strings)
    }
}

impl<'a, 'b> ToStrings<(String, Option<String>)> for (&'a str, Option<&'b str>) {
    fn to_strings(&self) -> (String, Option<String>) {
        (self.0.to_string(), self.1.map(String::from))
    }
}

impl<'a> ToStrings<Vec<String>> for [&'a str] {
    fn to_strings(&self) -> Vec<String> {
        self.iter()
            .map(|s| s.to_string())
            .collect()
    }
}

impl<'a> ToStrings<Vec<(String, String)>> for [(&'a str, &'a str)] {
    fn to_strings(&self) -> Vec<(String, String)> {
        self.iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect()
    }
}

impl<'a, 'b> ToStrings<Vec<(String, Vec<String>)>> for [(&'a str, &'b [&'b str])] {
    fn to_strings(&self) -> Vec<(String, Vec<String>)> {
        ToStringsCollect::to_strings_collect(self)
    }
}

impl<'a, 'b> ToStringsCollect for [(&'a str, &'b[&'b str])] {
    type Item = (String, Vec<String>);

    fn to_strings_collect<T>(&self) -> T
    where
        T: FromIterator<Self::Item>,
    {
        self.iter()
            .map(|x| (x.0.to_string(), x.1.to_strings()))
            .collect()
    }
}

#[macro_export]
macro_rules! vec_s {
    ($elem:expr; $n:expr) => (
        vec![$elem.to_string(), $n]
    );
    ($($x:expr),*) => (
        vec![$($x.to_string()),*]
    );
    ($($x:expr,)*) => (vec![$($x),*])
}


pub trait AsStrs<T> {
    fn as_strs(&self) -> T;
}

impl<'a, 'b> AsStrs<(&'a str, &'b str)> for (&'a String, &'b String) {
    fn as_strs(&self) -> (&'a str, &'b str) {
        (self.0.as_str(), self.1.as_str())
    }
}

impl<'a, 'b> AsStrs<Option<(&'a str, &'b str)>> for Option<(&'a String, &'b String)> {
    fn as_strs(&self) -> Option<(&'a str, &'b str)> {
        self.as_ref().map(AsStrs::<(&str, &str)>::as_strs)
    }
}


pub trait VecX<T: PartialEq> {
    fn remove_element(&mut self, item: &T) -> Option<T>;
}

impl<T: PartialEq> VecX<T> for Vec<T> {
    fn remove_element(&mut self, item: &T) -> Option<T> {
        let pos = self.iter().position(|x| *x == *item)?;
        Some(self.remove(pos))
    }
}