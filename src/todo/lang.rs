
pub trait ToStrings<T> {
    fn to_strings(&self) -> T;
}

impl<'a, 'b> ToStrings<(String, String)> for (&'a str, &'b str) {
    fn to_strings(&self) -> (String, String) {
        (self.0.to_string(), self.1.to_string())
    }
}

impl<'a> ToStrings<Vec<(String, String)>> for [(&'a str, &'a str)] {
    fn to_strings(&self) -> Vec<(String, String)> {
        self.iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect()
    }
}