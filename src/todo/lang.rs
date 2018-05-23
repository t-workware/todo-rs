
pub trait ToStrings<T> {
    fn to_strings(&self) -> T;
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

impl<'a> ToStrings<Vec<(String, String)>> for [(&'a str, &'a str)] {
    fn to_strings(&self) -> Vec<(String, String)> {
        self.iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect()
    }
}