
pub fn map_str<T, F: FnOnce(&T) -> &str>(source: &Option<T>, f: F) -> &str {
    source.as_ref().map(f).unwrap_or_default()
}